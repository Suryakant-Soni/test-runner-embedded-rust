// Result of the test after run
use crate::logger::types::Logger;
pub enum Outcome {
    Pass,
    Fail(&'static str),
    Error(&'static str),
}

// a single test case instance
#[derive(Copy, Clone)]
pub struct TestCase {
    name: &'static str,
    f: fn(&mut dyn Logger) -> Outcome,
}
impl TestCase {
    pub fn new(name: &'static str, f: fn(&mut dyn Logger) -> Outcome) -> TestCase {
        TestCase { name, f }
    }
    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get_f(&self) -> fn(&mut dyn Logger) -> Outcome {
        self.f
    }
}
