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

#[cfg(test)]
mod tests {
    use crate::{parse::parse_string, table::DataTypes};

    #[test]
    fn test_parse_string() {
        let x = "1".to_string();
        let y = "hello".to_string();
        let z = "3.0".to_string();

        let a = parse_string(&x).unwrap();
        let b = parse_string(&y).unwrap();
        let c = parse_string(&z).unwrap();

        let p = DataTypes::IntType(1);
        let q = DataTypes::StringType("hello".to_string());
        let r = DataTypes::FloatType(3.0);

        assert_eq!(a, p);
        assert_eq!(b, q);
        assert_eq!(c, r)
    }
}
