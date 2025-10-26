use crate::logger::types::Logger;
use crate::registry::types::{Outcome, TestCase};
pub struct Registry<const N: usize> {
    pub tests: [Option<TestCase>; N],
    pub len: usize,
}

impl<const N: usize> Registry<N> {
    pub const fn new() -> Self {
        Self {
            tests: [None; N],
            len: 0,
        }
    }

    // register a test case
    pub fn register(&mut self, test: TestCase) -> Result<(), ()> {
        if self.len == N {
            return Err(());
        }
        self.tests[self.len] = Some(test);
        self.len += 1;
        Ok(())
    }

    // run all registered tests
    pub fn run_all(&self, log: &mut dyn Logger) {
        for i in 0..self.len {
            let test = self.tests[i].as_ref().unwrap();
            log.log("RUN ");
            log.log(test.name);
            log.log(" ... ");
            match (test.f)(log) {
                Outcome::Pass => log.log("ok\n"),
                Outcome::Fail(m) => {
                    log.log("Fail\n");
                    log.log(m);
                    log.log("\n");
                }
                Outcome::Error(m) => {
                    log.log("Error: ");
                    log.log(m);
                    log.log("\n");
                }
            }
        }
    }
}
