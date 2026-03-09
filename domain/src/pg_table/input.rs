use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct PgTableInputDTO {
    table: String,
    schema: String,
}

impl PgTableInputDTO {
    pub fn parse(table_name: String, schema: String) -> Result<Self, String> {
        Ok(Self {
            table: validate_name(&table_name)?,
            schema: validate_name(&schema)?,
        })
    }
    pub fn get_table(&self) -> String {
        self.table.to_owned()
    }
    pub fn get_schema(&self) -> String {
        self.schema.to_owned()
    }
}

fn validate_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("identifier cannot be empty".to_string());
    }

    // Only allow alphanumeric and underscore
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || ['_', '-', ' '].contains(&c))
    {
        return Err(format!("identifier contains invalid characters: {}", name));
    }

    // Must not start with a number
    if name.chars().next().unwrap().is_numeric() {
        return Err(format!("identifier cannot start with a number: {}", name));
    }

    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use crate::pg_table::PgTableInputDTO;

    #[test]
    fn parse_pg_table_works() {
        let t = PgTableInputDTO::parse("table".into(), "schema".into())
            .expect("failed to parse table name");
        assert_eq!(&t.get_schema(), "schema");
        assert_eq!(&t.get_table(), "table")
    }

    #[test]
    fn parse_rejects_invalid_chars() {
        assert!(
            PgTableInputDTO::parse("table'; DROP TABLE users--".into(), "schema".into()).is_err()
        );
        assert!(PgTableInputDTO::parse("table".into(), "schema; DROP TABLE".into()).is_err());
    }

    #[test]
    fn parse_rejects_starts_with_number() {
        assert!(PgTableInputDTO::parse("123table".into(), "schema".into()).is_err());
    }

    #[test]
    fn parse_rejects_empty() {
        assert!(PgTableInputDTO::parse("".into(), "schema".into()).is_err());
        assert!(PgTableInputDTO::parse("table".into(), "".into()).is_err());
    }
}
