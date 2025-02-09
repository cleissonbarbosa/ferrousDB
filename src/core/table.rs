use serde::{Deserialize, Serialize};

use super::row::Row;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Constraint {
    NotNull,
    Unique,
    PrimaryKey,
    ForeignKey {
        ref_table: String,
        ref_column: String,
    },
    Check(String),  // Expression to check
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
    pub constraints: Vec<Constraint>,
}

impl ColumnSchema {
    pub fn new(name: String, data_type: String) -> Self {
        ColumnSchema { 
            name, 
            data_type,
            constraints: Vec::new(),
        }
    }

    pub fn with_constraints(name: String, data_type: String, constraints: Vec<Constraint>) -> Self {
        ColumnSchema {
            name,
            data_type,
            constraints,
        }
    }
}

impl std::fmt::Display for ColumnSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.data_type)?;
        for constraint in &self.constraints {
            write!(f, " {:?}", constraint)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for ColumnSchema {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(std::fmt::Error);
        }
        let name = parts[0].to_string();
        let data_type = parts[1].to_string();
        
        let mut constraints = Vec::new();
        for &part in &parts[2..] {
            match part.to_uppercase().as_str() {
                "NOT NULL" => constraints.push(Constraint::NotNull),
                "UNIQUE" => constraints.push(Constraint::Unique),
                "PRIMARY KEY" => constraints.push(Constraint::PrimaryKey),
                _ => {}
            }
        }
        
        Ok(ColumnSchema { name, data_type, constraints })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Represents a table in the database.
pub struct Table {
    /// The name of the table.
    pub name: String,
    /// The columns of the table.
    pub schema: Vec<ColumnSchema>,
    /// The rows of the table.
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String, schema: Vec<ColumnSchema>) -> Self {
        Table {
            name,
            schema,
            rows: Vec::new(),
        }
    }

    pub fn get_page(&self, mut page_number: usize, page_size: usize) -> Option<Vec<&Row>> {
        if page_number > self.total_pages(page_size) {
            return None;
        }

        // if page_number < 0 {
        //     return Some(self.rows.iter().collect());
        // }

        if page_number == 0 {
            println!("WARN: Page start with 1 not 0");
            page_number = 1;
        }

        let start = (page_number - 1) * page_size;
        let end = start + page_size;
        Some(self.rows[start..end.min(self.rows.len())].iter().collect())
    }

    pub fn total_pages(&self, page_size: usize) -> usize {
        (self.rows.len() + page_size - 1) / page_size
    }
}
