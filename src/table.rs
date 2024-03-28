use crate::command::string_check;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub rows: Vec<Vec<DataTypes>>,
    pub n_columns: usize,
    pub columns: HashMap<String, DataTypes>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypes {
    IntType(isize),
    FloatType(f32),
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
        return DataTypes::IntType(0);
    }

    if string_check(current_statement, "float", true) {
        return DataTypes::FloatType(0.0);
    }

    if string_check(current_statement, "varchar", true) {
        return DataTypes::StringType(String::new());
    }

    DataTypes::Null
}

#[cfg(test)]
mod tests {
    use super::{match_data_type, DataTypes};

    #[test]
    fn test_match_data_type() {
        let a = match_data_type("int");
        let b = match_data_type("float");
        let c = match_data_type("varchar");
        let d = match_data_type("INT");
        let e = match_data_type("FLOAT");
        let f = match_data_type("VARCHAR");
        let g = match_data_type("GLORB");

        let u = DataTypes::IntType(0);
        let v = DataTypes::FloatType(0.0);
        let w = DataTypes::StringType(String::new());
        let x = DataTypes::Null;

        assert_eq!(a, u);
        assert_eq!(b, v);
        assert_eq!(c, w);
        assert_eq!(d, u);
        assert_eq!(e, v);
        assert_eq!(f, w);
        assert_eq!(g, x);
    }
}
