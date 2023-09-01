pub enum ScriptLanguage {
    JavaScript,
    Python,
}
pub struct ScriptInfo {
    pub code: String,
    pub language: ScriptLanguage,
}
