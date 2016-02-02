use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
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

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if *self == *other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (&Value::Number(a), &Value::Number(b)) =>
                    if a < b {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    },
            (&Value::Bool(ref a), &Value::Bool(ref b)) =>
                    if a < b {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    },
            (&Value::Str(ref a), &Value::Str(ref b)) =>
                    if a < b {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    },
            _ => None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(ref v) => write!(f, "{}", v),
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::Str(ref s) => write!(f, "{}", s),
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

    pub fn to_boolean(&self) -> bool {
        match *self {
            Value::Number(v)    => v != 0.0,
            Value::Str(ref v)   => v.len() > 0,
            Value::Bool(v)      => v,
            Value::Undefined    => false
        }
    }
}
