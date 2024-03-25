use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub rows: Vec<Vec<DataTypes>>,
    pub n_columns: u16,
    pub columns: Vec<String>,
}

#[derive(Debug)]
pub enum DataTypes {
    U32Type(u32),
    I32Type(i32),
    FloatType(f32),
    U8Type(u8),
    StringType(String),
}

#[derive(Debug)]
pub struct ColumnSchema {
    column_name: String,
    column_type: DataTypes,
}

pub struct DataBase {
    pub tables: HashMap<String, Table>,
    pub name: String,
}

pub struct Manifest {
    pub databases: HashMap<String, DataBase>,
}
