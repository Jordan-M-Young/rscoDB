use crate::command::string_check;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub rows: Vec<Vec<DataTypes>>,
    pub n_columns: usize,
    pub columns: HashMap<String, DataTypes>,
}

#[derive(Debug, Clone)]
pub enum DataTypes {
    U32Type(u32),
    I32Type(i32),
    FloatType(f32),
    U8Type(u8),
    StringType(String),
    Null,
}

#[derive(Debug)]
pub struct ColumnSchema {
    pub column_name: String,
    pub column_type: DataTypes,
}

#[derive(Debug, Clone)]
pub struct DataBase {
    pub tables: HashMap<String, Table>,
    pub name: String,
}

pub struct Manifest {
    pub databases: HashMap<String, DataBase>,
}

pub fn match_data_type(current_statement: &str) -> DataTypes {
    if string_check(current_statement, "int", true) {
        return DataTypes::I32Type(0);
    }

    if string_check(current_statement, "float", true) {
        return DataTypes::FloatType(0.0);
    }

    if string_check(current_statement, "varchar", true) {
        return DataTypes::StringType(String::new());
    }

    DataTypes::Null
}
