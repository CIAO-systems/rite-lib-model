//! Module for common XML structures

use serde::{Deserialize, Serialize};

/// A structure to store TCP/IP connection information to a database
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "connection")]
pub struct DatabaseConnection {
    #[serde(rename = "@host")]
    pub host: String,
    #[serde(rename = "@port")]
    pub port: u16,
    #[serde(rename = "@database")]
    pub database: String,
    #[serde(rename = "@user")]
    pub user: String,
    #[serde(rename = "@password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@uniqueFields")]
    pub unique_fields: Option<String>,
    pub create: Option<String>,
}

impl Table {
    pub fn get_unique_fields_as_vec(&self) -> Vec<String> {
        if let Some(ref input) = self.unique_fields {
            return input.split(',').map(|s| s.trim().to_string()).collect();
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    #[test]
    fn test_unique_fields_ser() {
        let table = Table {
            name: "Name".to_string(),
            create: Some("CREATE-Statement".to_string()),
            unique_fields: Some("field1,field2".to_string()),
        };

        let x = serde_xml_rs::to_string(&table);
        assert!(x.is_ok());
        if let Ok(x) = x {
            assert_eq!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Table name=\"Name\" uniqueFields=\"field1,field2\"><create>CREATE-Statement</create></Table>".to_string(),
            x
        );
            println!("{:?}", x);
        }
    }

    #[test]
    fn test_unique_fields_de() {
        let xml = r#"<table name="Name" uniqueFields="field1,field2">
            <create>CREATE-Statement</create>
        </table>"#;

        let x: Table = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!("Name", x.name);
        println!("{:?}", x);
    }

    #[test]
    fn test_unique_fields_as_vec_empty() {
        let xml = r#"<table name="Name"/>"#;

        let table: Table = serde_xml_rs::from_str(xml).unwrap();
        // println!("{:?}", table);
        let v = table.get_unique_fields_as_vec();
        assert!(v.is_empty());
    }

    #[test]
    fn test_unique_fields_as_vec_non_empty() {
        let xml = r#"<table name="Name" uniqueFields="a,b,c"/>"#;

        let table: Table = serde_xml_rs::from_str(xml).unwrap();
        // println!("{:?}", table);
        let v = table.get_unique_fields_as_vec();
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(0).unwrap(), "a");
        assert_eq!(v.get(1).unwrap(), "b");
        assert_eq!(v.get(2).unwrap(), "c");
    }
}
