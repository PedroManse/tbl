mod schema {
    enum SQLType {
        Text,
        Num,
        Id,
    }

    struct Column {
        name: String,
        sqltype: SQLType,
    }

    struct Table {
        columns: Vec<Column>
    }
}

mod dump {
    enum SQLValue {
        Text(String),
        Num(f64),
        Id(i64),
    }
    struct Table {

    }
}


#[derive(Debug)]
pub enum Value {
    Str(String),
    Num(f64),
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

