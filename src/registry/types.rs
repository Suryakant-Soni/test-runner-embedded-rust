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
    pub name: &'static str,
    pub f: fn(&mut dyn Logger) -> Outcome,
}
