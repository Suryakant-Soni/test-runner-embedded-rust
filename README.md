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

Insights from project -

1. `static mut` is unsafe, it need unsafe block. since rust rules doesnt allow more than one mutable references of any
   object at a time.since compiler restriction
   are not possible in unsafe, it can have possible bugs.
2. `StaticCell` ensures rust rules and unsafe block is not needed. its initialization is via init method which cannot be
   called again, you use .get and .get_mut for getting references,
   you cannot call .get_mut again if the first object is alive in the scope
3. using a declarative macro to easily register a new test case - test_case!
4. kept attributes as private for Registry main object and test case object as well, using apis for construction and get
   attribute calls.