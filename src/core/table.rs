use serde::{Serialize, Deserialize};

use super::row::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}