use crate::{
    config::GuardConfig,
    output::{json_payload, markdown_payload, pretty_payload, sarif_payload},
    meta::CheckMeta,
};

pub mod config;
pub mod output;
pub mod meta;

pub fn main() {
    // clap definitions and main() logic
}