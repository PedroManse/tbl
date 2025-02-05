use std::collections::HashMap;

use crate::parse::{RowItem, Value};
use crate::TableDef;
use super::parse as prim;

pub fn prepare(tables: Vec<prim::Table>) {
    for table in tables {
        let mut td = TableDef::new(table.name);
        for row in table.rows {
            td.merge_row(&row.items);
        }
        println!("{td:?}");
    }
}

fn row_to_hash(row: Row) -> HashMap<String, Value> {
}
