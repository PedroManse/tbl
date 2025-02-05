use tbl::*;

fn main() {
    //let user_def = TableDef{
    //    name: "User".into(),
    //    columns: vec![
    //        ColumnDef{
    //            name: "name".into(),
    //            typedef: TypeDef::Text,
    //        },
    //    ]
    //};
    //let user_table = TableVal::new(&user_def, vec![
    //    HashMap::from([
    //        ("name".into(), TypeVal::Text("uwu".into())),
    //    ])
    //]);
    //println!("{user_table:?}");
    let content = std::fs::read_to_string("acc.tbl").unwrap();
    let tokens = token::tokenize(&content);
    let tables = parse::parse(tokens);
    let tables = prepare::prepare(tables);
}

