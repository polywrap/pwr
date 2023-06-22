pub trait Prompter {
    fn confirm(&self, message: String) -> Result<bool, String>;
}

pub struct PrompterMock;
impl PrompterMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Prompter for PrompterMock {
    fn confirm(&self, _message: String) -> Result<bool, String> {
        Ok(true)
    }
}
