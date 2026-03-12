use quick_xml::{Reader, events::Event};

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

pub fn unzip_content(zip_data: &[u8]) -> zip::result::ZipResult<String> {
    let cursor = Cursor::new(zip_data);
    let mut archive = zip::ZipArchive::new(cursor)?;

    let mut file = archive.by_index(0)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub text: String,
    pub children: Vec<XmlNode>,
}

impl PartialOrd for XmlNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XmlNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by name
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {
                // If names are equal, compare by attributes
                // Convert to sorted vectors for consistent comparison
                let mut self_attrs: Vec<_> = self.attributes.iter().collect();
                let mut other_attrs: Vec<_> = other.attributes.iter().collect();
                self_attrs.sort_by_key(|(k, _)| *k);
                other_attrs.sort_by_key(|(k, _)| *k);
                self_attrs.cmp(&other_attrs)
            }
            other => other,
        }
    }
}

impl XmlNode {
    pub fn new(name: String) -> Self {
        XmlNode {
            name,
            attributes: HashMap::new(),
            text: String::new(),
            children: Vec::new(),
        }
    }
    pub fn find_child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|child| child.name == name)
    }
}

pub fn parse_xml_to_tree(xml_string: &str) -> Result<XmlNode, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_string);
    let mut buf = Vec::new();
    let mut stack: Vec<XmlNode> = Vec::new();
    let mut root: Option<XmlNode> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())?;
                let mut node = XmlNode::new(name);

                // Extract attributes
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = String::from_utf8(attr.key.as_ref().to_vec())?;
                    let value = String::from_utf8(attr.value.to_vec())?;
                    node.attributes.insert(key, value);
                }

                stack.push(node);
            }

            Event::Empty(ref e) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())?;
                let mut node = XmlNode::new(name);

                // Extract attributes for self-closing tags
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = String::from_utf8(attr.key.as_ref().to_vec())?;
                    let value = String::from_utf8(attr.value.to_vec())?;
                    node.attributes.insert(key, value);
                }

                // Add to parent or set as root
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root = Some(node);
                }
            }

            Event::End(_) => {
                if let Some(completed_node) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(completed_node);
                    } else {
                        root = Some(completed_node);
                    }
                }
            }

            Event::Text(e) => {
                let text = e.unescape()?.trim().to_string();
                if !text.is_empty()
                    && let Some(current_node) = stack.last_mut()
                {
                    current_node.text = text;
                }
            }

            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    root.ok_or("No root element found".into())
}

pub fn extract_node_xml(
    xml_string: &str,
    node_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_string);
    let mut buf = Vec::new();
    let mut depth = 0;
    let mut capture = false;
    let mut captured_xml = String::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.name().as_ref() == node_name.as_bytes() => {
                capture = true;
                depth = 1;
                // Reconstruct the opening tag
                captured_xml.push('<');
                captured_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                for attr in e.attributes() {
                    let attr = attr?;
                    captured_xml.push(' ');
                    captured_xml.push_str(std::str::from_utf8(attr.key.as_ref())?);
                    captured_xml.push_str("=\"");
                    captured_xml.push_str(std::str::from_utf8(&attr.value)?);
                    captured_xml.push('"');
                }
                captured_xml.push('>');
            }
            Event::Start(ref e) if capture => {
                depth += 1;
                captured_xml.push('<');
                captured_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                for attr in e.attributes() {
                    let attr = attr?;
                    captured_xml.push(' ');
                    captured_xml.push_str(std::str::from_utf8(attr.key.as_ref())?);
                    captured_xml.push_str("=\"");
                    captured_xml.push_str(std::str::from_utf8(&attr.value)?);
                    captured_xml.push('"');
                }
                captured_xml.push('>');
            }
            Event::End(ref e) if capture => {
                captured_xml.push_str("</");
                captured_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                captured_xml.push('>');
                depth -= 1;
                if depth == 0 {
                    return Ok(captured_xml);
                }
            }
            Event::Empty(ref e) if capture => {
                captured_xml.push('<');
                captured_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                for attr in e.attributes() {
                    let attr = attr?;
                    captured_xml.push(' ');
                    captured_xml.push_str(std::str::from_utf8(attr.key.as_ref())?);
                    captured_xml.push_str("=\"");
                    captured_xml.push_str(std::str::from_utf8(&attr.value)?);
                    captured_xml.push('"');
                }
                captured_xml.push_str("/>");
            }
            Event::Text(e) if capture => {
                captured_xml.push_str(&e.unescape()?);
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Err("Node not found".into())
}

/// Extracts renderer-v2 XML from a QGIS style file
pub fn extract_renderer_v2(style_xml: &str) -> Result<String, Box<dyn std::error::Error>> {
    extract_node_xml(style_xml, "renderer-v2")
}

/// Extracts all style elements from a QGIS style file (all children of <qgis> root element)
/// Excludes structural/configuration elements that should not be copied
pub fn extract_all_style_elements(style_xml: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Elements that should NOT be extracted as they're structural, not styling
    // This includes form configuration, metadata, and elements that MapLayer already serializes
    const EXCLUDED_ELEMENTS: &[&[u8]] = &[
        // Elements MapLayer serializes
        b"flags",
        b"blendMode",
        b"temporal",
        b"expressionfields",
        // Form/attribute configuration
        b"editable",
        b"labelOnTop",
        b"reuseLastValue",
        b"aliases",
        b"defaults",
        b"constraints",
        b"constraintExpressions",
        b"attributeactions",
        b"attributetableconfig",
        b"fieldConfiguration",
        b"splitPolicies",
        b"editform",
        b"editforminit",
        b"editforminitcodesource",
        b"editforminitfilepath",
        b"editforminitcode",
        b"featformsuppress",
        b"editorlayout",
        b"dataDefinedFieldProperties",
        b"widgets",
        b"previewExpression",
        // Structural/metadata elements
        b"selection",
        b"customproperties",
        b"geometryOptions",
        b"referencedLayers",
        b"referencingLayers",
        b"layerGeometryType",
        b"extent",
        b"wgs84extent",
        b"srs",
        b"mapTip",
        b"legend",
        b"featureBlendMode",
        b"layerOpacity",
        b"elevation",
        b"conditionalstyles",
        b"storedexpressions",
        // Diagram configuration
        b"DiagramLayerSettings",
        b"LinearlyInterpolatedDiagramRenderer",
    ];

    let mut reader = Reader::from_str(style_xml);
    let mut buf = Vec::new();
    let mut depth = 0;
    let mut inside_qgis = false;
    let mut skip_element = false;
    let mut skip_depth = 0;
    let mut style_content = String::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.name().as_ref() == b"qgis" => {
                inside_qgis = true;
                depth = 1;
            }
            Event::Start(ref e) if inside_qgis && depth >= 1 => {
                // Check if we should skip this element
                if depth == 1 && EXCLUDED_ELEMENTS.contains(&e.name().as_ref()) {
                    skip_element = true;
                    skip_depth = depth + 1;
                }

                if !skip_element {
                    style_content.push('<');
                    style_content.push_str(std::str::from_utf8(e.name().as_ref())?);
                    for attr in e.attributes() {
                        let attr = attr?;
                        style_content.push(' ');
                        style_content.push_str(std::str::from_utf8(attr.key.as_ref())?);
                        style_content.push_str("=\"");
                        style_content.push_str(std::str::from_utf8(&attr.value)?);
                        style_content.push('"');
                    }
                    style_content.push('>');
                }
                depth += 1;
            }
            Event::End(ref e) if inside_qgis && e.name().as_ref() == b"qgis" => {
                break;
            }
            Event::End(ref e) if inside_qgis => {
                depth -= 1;

                // Check if we're exiting a skipped element
                if skip_element && depth < skip_depth {
                    skip_element = false;
                    skip_depth = 0;
                    continue;
                }

                if !skip_element {
                    style_content.push_str("</");
                    style_content.push_str(std::str::from_utf8(e.name().as_ref())?);
                    style_content.push('>');
                }
            }
            Event::Empty(ref e) if inside_qgis && depth > 1 => {
                if !skip_element {
                    style_content.push('<');
                    style_content.push_str(std::str::from_utf8(e.name().as_ref())?);
                    for attr in e.attributes() {
                        let attr = attr?;
                        style_content.push(' ');
                        style_content.push_str(std::str::from_utf8(attr.key.as_ref())?);
                        style_content.push_str("=\"");
                        style_content.push_str(std::str::from_utf8(&attr.value)?);
                        style_content.push('"');
                    }
                    style_content.push_str("/>");
                }
            }
            Event::Text(e) if inside_qgis && !skip_element => {
                style_content.push_str(&e.unescape()?);
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(style_content)
}

/// Inserts renderer-v2 XML into a QGIS project for the specified layer
pub fn insert_renderer_v2_into_project(
    project_xml: &str,
    renderer_v2: &str,
    layer_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Find the maplayer that contains the matching layername
    let mut search_pos = 0;

    loop {
        // Find next <maplayer> tag
        let maplayer_start = match project_xml[search_pos..].find("<maplayer") {
            Some(pos) => search_pos + pos,
            None => return Err("Layer not found in project".into()),
        };

        // Find the end of this maplayer
        let maplayer_end = match project_xml[maplayer_start..].find("</maplayer>") {
            Some(pos) => maplayer_start + pos,
            None => return Err("Malformed maplayer tag".into()),
        };

        let maplayer_content = &project_xml[maplayer_start..maplayer_end];

        // Check if this maplayer has the matching layername
        if let Some(layername_start) = maplayer_content.find("<layername>") {
            let layername_value_start = layername_start + "<layername>".len();
            if let Some(layername_end) =
                maplayer_content[layername_value_start..].find("</layername>")
            {
                let layername_value =
                    &maplayer_content[layername_value_start..layername_value_start + layername_end];

                if layername_value == layer_name {
                    // Found the right layer! Insert renderer-v2 before </maplayer>
                    let before = &project_xml[..maplayer_end];
                    let after = &project_xml[maplayer_end..];

                    return Ok(format!("{}\n    {}\n  {}", before, renderer_v2, after));
                }
            }
        }

        // Move past this maplayer and continue searching
        search_pos = maplayer_end + "</maplayer>".len();
        if search_pos >= project_xml.len() {
            return Err(format!("Layer '{}' not found in project", layer_name).into());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::qgis::{helpers::extract_renderer_v2, tests::test_helpers::xml_comparison};

    #[test]
    fn extract_renderer_v2_works() {
        let style_xml = include_str!("layer/examples/style_file.xml");
        let renderer_v2 = extract_renderer_v2(style_xml).expect("failed to extract renderer v2");
        let expected = include_str!("layer/examples/rendere_v2.xml");
        xml_comparison(&renderer_v2, expected, None);
    }
}
