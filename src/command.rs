// use crate::table::{DataTypes, Table};

pub enum CommandResult {
    CommandSuccess,
    CommandFailure,
    CommandQuit,
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    NullType,
    SelectType,
    CreateType,
    TableType,
    DataBaseType,
    InsertType,
    IntoType,
    ValueTypeType(String),
    ValuesType,
    FromType,
    NameType(String),
    MetaType(String),
    OperatorType(String),
    ShowType,
    DropType,
    ConnectType,
}

pub struct Statement {
    _statement_type: StatementType,
}

pub fn process_meta_command(command: &str) -> CommandResult {
    if command.trim() == ".quit".to_string() {
        return CommandResult::CommandQuit;
    } else {
        return CommandResult::CommandFailure;
    }
}

pub fn process_command(command: &str) -> CommandResult {
    if command.trim() == "select".to_string() || command.trim() == "SELECT".to_string() {
        let _statement = Statement {
            _statement_type: StatementType::SelectType,
        };

        println!("Selecting.....");

        return CommandResult::CommandSuccess;
    }

    if command.trim() == "insert".to_string() || command.trim() == "INSERT".to_string() {
        let _statement = Statement {
            _statement_type: StatementType::InsertType,
        };

        println!("Inserting.....");

        return CommandResult::CommandSuccess;
    }

    return CommandResult::CommandFailure;
}

pub fn match_statment(current_statement: &str) -> StatementType {
    let mut current_statement = current_statement.replace(",", "");
    current_statement = current_statement.replace("\r", "");
    current_statement = current_statement.replace("\n", "");

    if string_check(&current_statement, "select", true) {
        return StatementType::SelectType;
    }

    if string_check(&current_statement, "insert", true) {
        return StatementType::InsertType;
    }
    if string_check(&current_statement, "into", true) {
        return StatementType::IntoType;
    }
    if string_check(&current_statement, "values", true) {
        return StatementType::ValuesType;
    }
    if string_check(&current_statement, "from", true) {
        return StatementType::FromType;
    }

    if string_check(&current_statement, "", false) {
        return StatementType::NullType;
    }

    if current_statement.starts_with('.') {
        return StatementType::MetaType(current_statement);
    }

    if operator_check(&current_statement) {
        return StatementType::OperatorType(current_statement);
    }

    if string_check(&current_statement, "create", true) {
        return StatementType::CreateType;
    }
    if string_check(&current_statement, "table", true) {
        return StatementType::TableType;
    }
    if string_check(&current_statement, "connect", true) {
        return StatementType::ConnectType;
    }
    if data_type_check(&current_statement) {
        return StatementType::ValueTypeType(current_statement);
    }

    if string_check(&current_statement, "database", true) {
        return StatementType::DataBaseType;
    }

    if string_check(&current_statement, "show", true) {
        return StatementType::ShowType;
    }
    if string_check(&current_statement, "drop", true) {
        return StatementType::DropType;
    }

    return StatementType::NameType(current_statement);
}

pub fn string_check(main_str: &str, check_str: &str, caps: bool) -> bool {
    if main_str.trim() == check_str.to_string() {
        return true;
    }

    if main_str.trim().to_uppercase() == check_str.to_string().to_uppercase() && caps {
        return true;
    }

    return false;
}

pub fn data_type_check(current_statement: &str) -> bool {
    if string_check(current_statement, "int", true) {
        return true;
    }

    if string_check(current_statement, "float", true) {
        return true;
    }

    if string_check(current_statement, "varchar", true) {
        return true;
    }

    false
}

fn operator_check(current_statement: &str) -> bool {
    if string_check(current_statement, "==", false) {
        return true;
    }
    if string_check(current_statement, "!=", false) {
        return true;
    }
    if string_check(current_statement, ">", false) {
        return true;
    }
    if string_check(current_statement, "<", false) {
        return true;
    }
    if string_check(current_statement, "=<", false) {
        return true;
    }
    if string_check(current_statement, ">=", false) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::command::{data_type_check, match_statment, string_check};

    use super::{operator_check, StatementType};

    struct StatementPair {
        a: String,
        b: StatementType,
    }

    struct Pair {
        a: String,
        b: bool,
    }

    struct Quad {
        a: String,
        b: String,
        c: bool,
        d: bool,
    }

    impl StatementPair {
        fn new(a: &str, b: StatementType) -> StatementPair {
            StatementPair {
                a: a.to_string(),
                b,
            }
        }
    }

    impl Pair {
        fn new(a: &str, b: bool) -> Pair {
            Pair {
                a: a.to_string(),
                b,
            }
        }
    }

    impl Quad {
        fn new(a: &str, b: &str, c: bool, d: bool) -> Quad {
            Quad {
                a: a.to_string(),
                b: b.to_string(),
                c,
                d,
            }
        }
    }

    #[test]
    fn test_operator_check() {
        let pairs: Vec<Pair> = vec![
            Pair::new("==", true),
            Pair::new("!=", true),
            Pair::new(">", true),
            Pair::new("<", true),
            Pair::new("=<", true),
            Pair::new(">=", true),
            Pair::new("z", false),
            Pair::new("1", false),
            Pair::new("@", false),
        ];

        for pair in pairs {
            assert_eq!(operator_check(&pair.a), pair.b)
        }
    }

    #[test]
    fn test_data_type_check() {
        let pairs: Vec<Pair> = vec![
            Pair::new("int", true),
            Pair::new("INT", true),
            Pair::new("float", true),
            Pair::new("FLOAT", true),
            Pair::new("varchar", true),
            Pair::new("VARCHAR", true),
            Pair::new("VARCAR", false),
            Pair::new("floot", false),
            Pair::new("", false),
            Pair::new("text", false),
        ];

        for pair in pairs {
            assert_eq!(data_type_check(&pair.a), pair.b)
        }
    }

    #[test]
    fn test_string_check() {
        let quads: Vec<Quad> = vec![
            Quad::new("select", "select", false, true),
            Quad::new("SELECT", "select", false, false),
            Quad::new("select", "SELECT", true, true),
            Quad::new("INSERT", "select", true, false),
        ];

        for quad in quads {
            assert_eq!(string_check(&quad.a, &quad.b, quad.c), quad.d)
        }
    }

    #[test]
    fn test_match_statement() {
        // need to be exhaustive with statement types...
        let statement_pairs: Vec<StatementPair> = vec![
            StatementPair::new("select", StatementType::SelectType),
            StatementPair::new("from", StatementType::FromType),
            StatementPair::new(
                "table_name",
                StatementType::NameType("table_name".to_string()),
            ),
            StatementPair::new("TABLE", StatementType::TableType),
        ];

        for pair in statement_pairs {
            assert_eq!(match_statment(&pair.a), pair.b)
        }
    }
}
