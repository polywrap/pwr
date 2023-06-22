pub trait Logger {
    fn debug(&self, message: String) -> Result<(), String>;
    fn info(&self, message: String) -> Result<(), String>;
    fn warn(&self, message: String) -> Result<(), String>;
    fn error(&self, message: String) -> Result<(), String>;
}
pub struct LoggerMock;
impl LoggerMock {
    pub fn new() -> Self {
        Self {}
    }
}
impl Logger for LoggerMock {
    fn debug(&self, _message: String) -> Result<(), String> {
        // println!("DEBUG: {}", message);
        Ok(())
    }

    fn info(&self, message: String) -> Result<(), String> {
        println!("INFO {}", message);
        Ok(())
    }

    fn warn(&self, message: String) -> Result<(), String> {
        println!("WARN: {}", message);
        Ok(())
    }

    fn error(&self, message: String) -> Result<(), String> {
        println!("ERROR: {}", message);
        Ok(())
    }
}
