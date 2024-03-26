// use crate::table::{DataTypes, Table};

use std::string;

pub enum CommandResult {
    CommandSuccess,
    CommandFailure,
    CommandQuit,
}

#[derive(Debug)]
pub enum StatementType {
    NullType,
    SelectType,
    CreateType,
    TableType,
    DataBaseType,
    InsertType,
    IntoType,
    ValueTypeType(String),
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

    if main_str.trim() == check_str.to_string().to_uppercase() && caps {
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
