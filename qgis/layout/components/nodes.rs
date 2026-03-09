use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Nodes {
    #[serde(rename = "node")]
    pub nodes: Vec<Node>,
}

#[derive(Serialize)]
pub struct Node {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
}
