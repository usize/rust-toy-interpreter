use object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Object(Object),
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
                    a.is_nan() == b.is_nan()
                },
            (&Value::Str(ref a), &Value::Str(ref b)) => a == b,
            (&Value::Undefined, &Value::Undefined) => true,
            _ => false
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

    pub fn mul(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Value::Str("invalid operation".to_string())
        }
    }

    pub fn div(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Value::Str("invalid operation".to_string())
        }
    }
}
