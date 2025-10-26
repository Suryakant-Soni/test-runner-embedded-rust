// Provided by test-runner lib:
Outcome = Pass | Fail(&'static str) | Error(&'static str)
Logger = trait { log(&mut self, s: &str) }
TestCase = { name: &'static str, f: fn(&mut dyn Logger) -> Outcome }
RegistryN = fixed-size array of Option<TestCase>; register(); run_all()

// App writes:
fn my_test(log: &mut dyn Logger) -> Outcome {
// use assert_eq/assert_ne/assert_true/assert_false/...
assert_eq(some_app_fn(), 42)
}

// Harness wires:
REG.register(TestCase{name: "my_test", f: my_test});
REG.run_all(&mut logger);