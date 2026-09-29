use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SarifPayload {
    pub version: String,
    pub rules: Vec<Rule>,
    pub runs: Vec<Run>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub short_description: String,
    pub severity: Severity,
    pub message: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Run {
    pub ruleIndexes: Vec<u32>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub check: String,
    pub line: u32,
    pub column: u32,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

pub fn sarif_payload(check_meta: &CheckMeta, findings: Vec<Finding>) -> String {
    // SARIF renderer logic
}