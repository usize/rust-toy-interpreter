use std::fmt;
use object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Object(Object),
    Bool(bool),
    Undefined
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(a), &Value::Number(b)) =>
                if a == b {
                    // The two values are the same... unless they're 0 and -0.
                    a.is_sign_negative() == b.is_sign_negative()
                } else {
                    // The two values are different... unless they're both NaN.
                    a.is_nan() && b.is_nan()
                },
            (&Value::Bool(ref a), &Value::Bool(ref b)) => a == b,
            (&Value::Str(ref a), &Value::Str(ref b)) => a == b,
            (&Value::Undefined, &Value::Undefined) => true,
            _ => false
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(ref v) => write!(f, "{}", v),
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::Str(ref s) => write!(f, "{}", s),
            Value::Object(_)   => write!(f, "[Object]"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

impl Value {
    pub fn add(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (&Value::Str(ref a), Value::Str(ref b)) => Value::Str(a.clone() + b),
            _ => return Err("invalid operation".to_string())
        })
    }

    pub fn sub(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => return Err("invalid operation".to_string())
        })
    }

    pub fn mul(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => return Err("invalid operation".to_string())
        })
    }

    pub fn div(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => return Err("invalid operation".to_string())
        })
    }

    pub fn as_bool(&self) -> Value {
        match *self {
            Value::Number(ref v)    => Value::Bool(*v != 0 as f64),
            Value::Str(ref v)       => Value::Bool(v.len() > 0),
            Value::Bool(ref v)      => Value::Bool(*v == true),
            Value::Object(_)    => Value::Bool(true),
            Value::Undefined    => Value::Bool(false)
        }
    }
}
