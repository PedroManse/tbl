#[derive(Debug)]
pub enum Token {
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Colen,
    Comma,
    Identf(String),
    ValStr(String),
    ValNum(f64),
}

#[derive(Debug)]
enum State {
    // # ignore space #
    // [A-Za-z_ ] -> MakingTableName
    Nothing,

    // # trim space #
    // [A-Za-z_] -> MakingTableName
    // "[" -> InsideTable || commit [buf.trim() as Identf] [OpenSquare]
    MakingTableName { buf: String },

    // # ignore space #
    // "]" -> Nothing || commit CloseSquare
    // [0-9] -> MakingRowID
    InsideTable,

    // # ignore space #
    // [0-9] -> MakingRowID
    // "{" -> InsideRow || parse and commit [buf as ValNum] [OpenCurly]
    MakingRowId { buf: String },

    // # ignore space #
    // [A-Za-z_*] -> RowItemName
    InsideRow,

    // # trim space #
    // [A-Za-z_ ] -> RowItemName
    // ":" -> WaitingRowItemValue || commit [buf as Identf] [Colen]
    RowItemName { buf: String },

    // '"' -> RowItemValueStr
    // [0-9.] -> RowItemValueNum
    WaitingRowItemValue,

    // '"' -> RowItemValueStrDone || commit [buf as ValStr]
    // * -> RowItemValueStr
    RowItemValueStr { buf: String },

    // "," -> InsideRow || commit [Comma]
    // "}" -> InsideTable || commit [CloseCurly]
    RowItemValueStrDone,

    // [0-9.] -> RowItemValueNum
    // "," -> InsideRow || commit [buf as ValNum] [Comma]
    // "}" -> InsideTable || commit [CloseCurly]
    RowItemValueNum { buf: String },
}

pub fn tokenize(cont: &str) -> Vec<Token> {
    use State::*;
    let mut state = Nothing;
    let mut tokens = vec![];
    macro_rules! catch {
        (space) => { ' ' | '\t' | '\n' };
        (alpha) => { 'A'..='Z' | 'a'..='z' };
        (num) => { '0'..='9' | '.' };
        ( $( $c:tt ),* ) => {
            $(
                catch!($c)
            )|*
        };
    }
    for c in cont.chars() {
        state = match (state, c) {
            (Nothing, ch @ (catch!(alpha, space) | '_')) => MakingTableName {
                buf: ch.to_string(),
            },
            (MakingTableName { mut buf }, ch @ (catch!(alpha, num, space) | '_')) => {
                buf.push(ch);
                MakingTableName { buf }
            }
            (MakingTableName { buf }, '[') => {
                tokens.push(Token::Identf(buf.trim().to_owned()));
                tokens.push(Token::OpenSquare);
                InsideTable
            }
            (InsideTable, ']') => {
                tokens.push(Token::CloseSquare);
                Nothing
            }
            (InsideTable, ch @ catch!(num)) => MakingRowId {
                buf: ch.to_string(),
            },
            (MakingRowId { mut buf }, ch @ catch!(num)) => {
                buf.push(ch);
                MakingRowId { buf }
            }
            (MakingRowId { buf }, '{') => {
                let n = buf.parse().unwrap();
                tokens.push(Token::ValNum(n));
                tokens.push(Token::OpenCurly);
                InsideRow
            }
            (InsideRow, ch @ (catch!(alpha) | '*' | '_')) => RowItemName {
                buf: ch.to_string(),
            },
            (InsideRow, '}') => {
                tokens.push(Token::CloseCurly);
                InsideTable
            }
            (RowItemName { mut buf }, ch @ (catch!(alpha, space) | '_')) => {
                buf.push(ch);
                RowItemName { buf }
            }
            (RowItemName { buf }, ':') => {
                tokens.push(Token::Identf(buf.trim().to_owned()));
                tokens.push(Token::Colen);
                WaitingRowItemValue
            }
            (WaitingRowItemValue, '"') => RowItemValueStr { buf: String::new() },
            (RowItemValueStr { buf }, '"') => {
                tokens.push(Token::ValStr(buf));
                RowItemValueStrDone
            }
            (RowItemValueStr { mut buf }, ch) => {
                buf.push(ch);
                RowItemValueStr { buf }
            }
            (RowItemValueStrDone, ',') => {
                tokens.push(Token::Comma);
                InsideRow
            }
            (RowItemValueStrDone, '}') => {
                tokens.push(Token::CloseCurly);
                InsideTable
            }
            (WaitingRowItemValue, ch @ catch!(num)) => RowItemValueNum {
                buf: ch.to_string(),
            },
            (RowItemValueNum { mut buf }, ch @ catch!(num)) => {
                buf.push(ch);
                RowItemValueNum {
                    buf: ch.to_string(),
                }
            }
            (RowItemValueNum { buf }, ',') => {
                let num = buf.parse().unwrap();
                tokens.push(Token::ValNum(num));
                tokens.push(Token::Comma);
                InsideRow
            }
            (RowItemValueNum { buf }, '}') => {
                let num = buf.parse().unwrap();
                tokens.push(Token::ValNum(num));
                tokens.push(Token::CloseCurly);
                InsideTable
            }

            (s, catch!(space)) => s,
            (s, c) => {
                eprintln!("state: {s:?}");
                eprintln!("char: {c:?}");
                panic!()
            }
        }
    }
    tokens
}
