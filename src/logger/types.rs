pub trait Logger {
    fn log(&mut self, msg: &str);
}
