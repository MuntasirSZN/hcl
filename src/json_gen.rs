use crate::types::Command;
use serde_json::json;

pub struct JsonGenerator;

impl JsonGenerator {
    pub fn generate(cmd: &Command) -> String {
        let json = Self::command_to_json(cmd);
        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    fn command_to_json(cmd: &Command) -> serde_json::Value {
        let mut obj = json!({
            "name": cmd.name,
            "description": cmd.description,
            "usage": cmd.usage,
            "options": cmd.options.iter().map(|opt| {
                json!({
                    "names": opt.names.iter().map(|n| n.raw.clone()).collect::<Vec<_>>(),
                    "argument": opt.argument,
                    "description": opt.description,
                })
            }).collect::<Vec<_>>(),
        });

        if !cmd.subcommands.is_empty() {
            obj["subcommands"] = serde_json::json!(
                cmd.subcommands
                    .iter()
                    .map(|sub| {
                        json!({
                            "name": sub.name,
                            "description": sub.description,
                        })
                    })
                    .collect::<Vec<_>>()
            );
        }

        if !cmd.version.is_empty() {
            obj["version"] = json!(cmd.version);
        }

        obj
    }
}
