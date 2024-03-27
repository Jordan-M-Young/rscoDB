use crate::DataTypes;

pub fn parse_string(s: &str) -> Option<DataTypes> {
    if let Ok(i) = s.parse() {
        Some(DataTypes::IntType(i))
    } else if let Ok(f) = s.parse() {
        Some(DataTypes::FloatType(f))
    } else {
        Some(DataTypes::StringType(s.to_string()))
    }
}
