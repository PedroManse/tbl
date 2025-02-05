pub use std::collections::HashMap;
pub mod parse;
pub mod token;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDef { Id, Num, Text, }
#[derive(Debug)]
pub enum TypeVal { Id(i64), Num(i64), Text(String) }

impl From<&TypeVal> for TypeDef {
    fn from(value: &TypeVal) -> Self {
        use TypeVal as V;
        use TypeDef as T;
        match value {
            V::Id(_)=>T::Id,
            V::Num(_)=>T::Num,
            V::Text(_)=>T::Text,
        }
    }
}

#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub typedef: TypeDef,
}

#[derive(Debug)]
pub struct TableDef {
    pub name: String,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug)]
pub struct TableVal<'a> {
    pub def: &'a TableDef,
    pub rows: Vec<HashMap<String, TypeVal>>,
}

impl<'a> TableVal<'a> {
    pub fn new(def: &'a TableDef, rows: Vec<HashMap<String, TypeVal>>) -> TableVal<'a> {
        let ok = rows.first().map(|row|{
            for col in &def.columns {
                let Some(colv) = row.get(&col.name) else {
                    return false;
                };
                if col.typedef != colv.into() {
                    return false;
                }
            }
            true
        }).unwrap_or(true);
        if !ok {
            panic!("Mistaken types\ndef: {def:?}\nrows: {rows:?}")
        }
        TableVal{ def, rows }
    }
}
