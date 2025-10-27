use static_cell::StaticCell;
use test_runner::assert::functions::{assert_eq, assert_true};
use test_runner::logger::types::Logger;
use test_runner::registry::engine::Registry;
use test_runner::registry::types::Outcome;
use test_runner::test_case;
static REG_STORAGE: StaticCell<Registry<64>> = StaticCell::new();
struct HostLogger;
impl Logger for HostLogger {
    fn log(&mut self, _msg: &str) {
        println!("{}", _msg);
    }
}

fn t_add(_: &mut dyn Logger) -> Outcome {
    assert_eq(2 + 2, 4)
}

fn t_truth(_: &mut dyn Logger) -> Outcome {
    assert_true(4 < 10 - 3)
}
fn main() {
    // logger as per stdout
    let mut log = HostLogger;
    let reg = REG_STORAGE.init(Registry::new());
    unsafe {
        reg.register(test_case!("math:add", t_add)).ok();
        reg.run_all(&mut log);
    }
}
