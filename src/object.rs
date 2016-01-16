trait Object {
    fn set_property(&mut self, name: String, value: Value);
    fn get_property(&mut self, name: String) -> Value;
}

trait Callable {
    fn call(&self, args: [Value]);
}
