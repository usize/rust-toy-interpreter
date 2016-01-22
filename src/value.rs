use object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Object(Object),
    Undefined,
    Error(&'static str),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::Number(a), &Value::Number(b)) => a == b,
            (&Value::Str(ref a), &Value::Str(ref b)) => a == b,
            (&Value::Undefined, &Value::Undefined) => true,
            _ => false
        }
    }
}

impl Value {
    pub fn add(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => Value::Error("invalid operation")
        }
    }

    pub fn sub(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => Value::Error("invalid operation")
        }
    }

    pub fn mul(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => Value::Error("invalid operation")
        }
    }

    pub fn div(&self, b: Value) -> Value {
        match (self, b) {
            (&Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => Value::Error("invalid operation")
        }
    }
}
