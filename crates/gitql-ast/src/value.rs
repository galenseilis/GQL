use crate::types::DataType;

#[derive(PartialEq, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
    Date(i64),
    Null,
}

impl Value {
    pub fn eq(&self, other: &Value) -> bool {
        if self.data_type() != other.data_type() {
            return false;
        }

        return match self.data_type() {
            DataType::Any => true,
            DataType::Text => self.as_text() == other.as_text(),
            DataType::Number => self.as_number() == other.as_number(),
            DataType::Boolean => self.as_bool() == other.as_bool(),
            DataType::Date => self.as_date() == other.as_date(),
            DataType::Undefined => true,
            DataType::Null => true,
        };
    }

    pub fn data_type(&self) -> DataType {
        return match self {
            Value::Number(_) => DataType::Number,
            Value::Text(_) => DataType::Text,
            Value::Boolean(_) => DataType::Boolean,
            Value::Date(_) => DataType::Date,
            Value::Null => DataType::Null,
        };
    }

    pub fn literal(&self) -> String {
        return match self {
            Value::Number(i) => i.to_string(),
            Value::Text(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Date(d) => d.to_string(),
            Value::Null => "Null".to_string(),
        };
    }

    pub fn as_number(&self) -> i64 {
        if let Value::Number(n) = self {
            return *n;
        }
        return 0;
    }

    pub fn as_text(&self) -> String {
        if let Value::Text(s) = self {
            return s.to_string();
        }
        return "".to_owned();
    }

    pub fn as_bool(&self) -> bool {
        if let Value::Boolean(b) = self {
            return *b;
        }
        return false;
    }

    pub fn as_date(&self) -> i64 {
        if let Value::Date(d) = self {
            return *d;
        }
        return 0;
    }
}
