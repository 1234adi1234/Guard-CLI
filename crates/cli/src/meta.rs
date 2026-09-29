use std::collections::HashMap;

pub struct CheckMeta {
    pub checks: HashMap<String, Check>,
}

#[derive(Debug, Clone)]
pub struct Check {
    pub name: String,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

pub fn load_check_meta() -> CheckMeta {
    // Hand-written CheckMeta table logic
}