pub use std::collections::HashMap;
use std::collections::HashSet;
pub mod parse;
pub mod token;
pub mod prepare;
use self::parse::{Value, RowItem};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TypeDef {
    Num,
    Str,
    Id,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub typedef: TypeDef,
}

#[derive(Debug)]
pub struct TableDef {
    pub name: String,
    pub columns: HashSet<ColumnDef>,
}

#[derive(Debug)]
pub struct TableVal<'a> {
    pub def: &'a TableDef,
    pub rows: Vec<HashMap<String, Value>>,
}

impl From<&Value> for TypeDef {
    fn from(value: &Value) -> Self {
        use TypeDef as T;
        use Value as V;
        match value {
            V::Num(_) => T::Num,
            V::Str(_) => T::Str,
        }
    }
}
impl From<Value> for TypeDef {
    fn from(value: Value) -> Self {
        (&value).into()
    }
}

impl TableDef {
    pub fn new(name: String) -> Self {
        TableDef{name, columns: HashSet::new()}
    }
    pub fn merge_row(&mut self, rows: Vec<RowItem>) {
        for item in rows {
            let c = ColumnDef{
                typedef: match item.value {
                    Value::Str(_)=>TypeDef::Str,
                    Value::Num(_) if item.name.starts_with('*')=>TypeDef::Id,
                    Value::Num(_)=>TypeDef::Num,
                },
                name: item.name,
            };
            self.columns.insert(c);
        }
    }
}

impl<'a> TableVal<'a> {
    pub fn new(def: &'a TableDef, rows: Vec<HashMap<String, Value>>) -> TableVal<'a> {
        let ok = rows
            .first()
            .map(|row| {
                for col in &def.columns {
                    let Some(colv) = row.get(&col.name) else {
                        return false;
                    };
                    if col.typedef != colv.into() {
                        return false;
                    }
                }
                true
            })
            .unwrap_or(true);
        if !ok {
            panic!("Mistaken types\ndef: {def:?}\nrows: {rows:?}")
        }
        TableVal { def, rows }
    }
}
