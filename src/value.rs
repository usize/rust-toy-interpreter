#[derive(Debug, Copy, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Error(&'static str),
}

impl Value {

    pub fn add(&self, b: Value) -> Value {
        match *self {
            Value::Int(a)      => Value::int_add(a, b),
            Value::Float(a)    => Value::float_add(a, b),
            _                  => Value::Error("invalid operation")
        }
    }

    fn int_add(a: i32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Int(a + b),
            Value::Float(b) => Value::Float(a as f32 + b),
            _               => Value::Error("invalid operation")
        }
    }

    fn float_add(a: f32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Float(a + b as f32),
            Value::Float(b) => Value::Float(a + b),
            _               => Value::Error("invalid operation")
        }
    }

    pub fn sub(&self, b: Value) -> Value {
        match *self {
            Value::Int(a)      => Value::int_sub(a, b),
            Value::Float(a)    => Value::float_sub(a, b),
            _                  => Value::Error("invalid operation")
        }
    }

    fn int_sub(a: i32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Int(a - b),
            Value::Float(b) => Value::Float(a as f32 - b),
            _               => Value::Error("invalid operation")
        }
    }

    fn float_sub(a: f32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Float(a - b as f32),
            Value::Float(b) => Value::Float(a - b),
            _               => Value::Error("invalid operation")
        }
    }

    pub fn mul(&self, b: Value) -> Value {
        match *self {
            Value::Int(a)      => Value::int_mul(a, b),
            Value::Float(a)    => Value::float_mul(a, b),
            _                  => Value::Error("invalid operation")
        }
    }

    fn int_mul(a: i32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Int(a * b),
            Value::Float(b) => Value::Float(a as f32 * b),
            _               => Value::Error("invalid operation")
        }
    }

    fn float_mul(a: f32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Float(a * b as f32),
            Value::Float(b) => Value::Float(a * b),
            _               => Value::Error("invalid operation")
        }
    }

    pub fn div(&self, b: Value) -> Value {
        match *self {
            Value::Int(a)      => Value::int_div(a, b),
            Value::Float(a)    => Value::float_div(a, b),
            _                  => Value::Error("invalid operation")
        }
    }

    fn int_div(a: i32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Int(a / b),
            Value::Float(b) => Value::Float(a as f32 / b),
            _               => Value::Error("invalid operation")
        }
    }

    fn float_div(a: f32, b: Value) -> Value {
        match b {
            Value::Int(b)   => Value::Float(a / b as f32),
            Value::Float(b) => Value::Float(a / b),
            _               => Value::Error("invalid operation")
        }
    }

}

