#[macro_export]
macro_rules! test_case {
    ($name:expr, $fn:path) => {
        $crate::registry::types::TestCase {
            name: $name,
            f: $fn,
        }
    };
}
