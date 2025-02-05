use super::token::Token;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Num(f64),
}

#[derive(Debug)]
struct PartialRow {
    id: i64,
    items: Vec<RowItem>,
}

#[derive(Debug)]
struct PartialRowItem {
    name: String,
}

#[derive(Debug)]
pub struct RowItem {
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub struct Row {
    pub id: i64,
    pub items: Vec<RowItem>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub rows: Vec<Row>,
}

#[derive(Debug)]
struct PartialTable {
    name: String,
    rows: Vec<Row>,
}

#[derive(Debug)]
enum State {
    //Identf -> MakingTable
    Nothing,

    // "[" -> InsideTable
    MakingTable {
        table_name: String,
    },

    // [:num:] -> MakingRow
    // "]" -> Nothing
    InsideTable {
        table: PartialTable,
    },

    // "{" -> InsideRow
    MakingRow {
        table: PartialTable,
        row_id: i64,
    },

    // "}" -> InsideTable
    // Identf -> MakingRowValue
    InsideRow {
        table: PartialTable,
        row: PartialRow,
    },

    // ":" -> InsideRowValue
    MakingRowValue {
        table: PartialTable,
        row: PartialRow,
        rowitem: PartialRowItem,
    },

    // [:value:] -> DoneRowValue
    InsideRowValue {
        table: PartialTable,
        row: PartialRow,
        rowitem: PartialRowItem,
    },

    // "," -> InsideRow
    // "}" -> InsideTable
    DoneRowValue {
        table: PartialTable,
        row: PartialRow,
        rowitem: RowItem,
    },
}

pub fn x(cont: &str) -> Vec<Table> {
    use State::*;
    use Token::*;
    let tokens = super::token::tokenize(cont);

    let mut tables: Vec<Table> = vec![];
    let mut state = State::Nothing;
    for token in tokens {
        state = match (state, token) {
            (Nothing, Identf(name)) => MakingTable { table_name: name },
            (MakingTable { table_name }, OpenSquare) => InsideTable {
                table: PartialTable {
                    name: table_name,
                    rows: vec![],
                },
            },
            (InsideTable { table }, CloseSquare) => {
                tables.push(Table {
                    name: table.name,
                    rows: table.rows,
                });
                Nothing
            }
            (InsideTable { table }, ValNum(id)) => MakingRow {
                table,
                row_id: id as i64,
            },
            (MakingRow { table, row_id }, OpenCurly) => InsideRow {
                table,
                row: PartialRow {
                    id: row_id,
                    items: vec![],
                },
            },
            (InsideRow { mut table, row }, CloseCurly) => {
                table.rows.push(Row {
                    id: row.id,
                    items: row.items,
                });
                InsideTable { table }
            }
            (InsideRow { table, row }, Identf(name)) => MakingRowValue {
                table,
                row,
                rowitem: PartialRowItem { name },
            },
            (
                MakingRowValue {
                    table,
                    row,
                    rowitem,
                },
                Colen,
            ) => InsideRowValue {
                table,
                row,
                rowitem,
            },
            (
                InsideRowValue {
                    table,
                    row,
                    rowitem,
                },
                ValNum(n),
            ) => DoneRowValue {
                table,
                row,
                rowitem: RowItem {
                    name: rowitem.name,
                    value: Value::Num(n),
                },
            },
            (
                InsideRowValue {
                    table,
                    row,
                    rowitem,
                },
                ValStr(s),
            ) => DoneRowValue {
                table,
                row,
                rowitem: RowItem {
                    name: rowitem.name,
                    value: Value::Str(s),
                },
            },
            (
                DoneRowValue {
                    table,
                    mut row,
                    rowitem,
                },
                Comma,
            ) => {
                row.items.push(rowitem);
                InsideRow { table, row }
            }
            (
                DoneRowValue {
                    mut table,
                    mut row,
                    rowitem,
                },
                CloseCurly,
            ) => {
                row.items.push(rowitem);
                table.rows.push(Row {
                    id: row.id,
                    items: row.items,
                });
                InsideTable { table }
            }
            (s, t) => {
                eprintln!("state: {s:?}");
                eprintln!("token: {t:?}");
                panic!()
            }
        };
    }
    tables
}
