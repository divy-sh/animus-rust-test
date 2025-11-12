use std::{collections::HashMap, sync::OnceLock};

use crate::resp::resp::{Resp, Typ, Value};

pub struct Command {
    pub func: fn(Vec<Resp>) -> Resp,
    pub doc: &'static str,
    pub arity: i32,
    pub flags: &'static [&'static str],
    pub first_key: i32,
    pub last_key: i32,
    pub step: i32,
}

// Static global registry
static COMMANDS: OnceLock<HashMap<&'static str, Command>> = OnceLock::new();

// Example command implementations
fn ping(_args: Vec<Resp>) -> Resp {
    Resp {
        val: Value::Str("PONG".to_string()),
        typ: Typ::STRING,
    }
}

fn info(args: Vec<Resp>) -> Resp {
    match args[1].val {
        Value::Str(ref s) if s.to_uppercase() == "SAVE" => {
            return Resp {
                val: Value::Arr([Resp {
                    typ: Typ::BULK,
                    val: Value::Str(s.to_uppercase().to_string()),
                },
                Resp {
                    typ: Typ::BULK,
                    val: Value::Str("".to_string()),
                }
                ].to_vec()),
                typ: Typ::BULK,
            };
        }
        Value::Str(ref s) if s.to_uppercase() == "APPENDONLY" => {
            return Resp {
                val: Value::Arr([Resp {
                    typ: Typ::BULK,
                    val: Value::Str(s.to_uppercase().to_string()),
                },
                Resp {
                    typ: Typ::BULK,
                    val: Value::Str("no".to_string()),
                }
                ].to_vec()),
                typ: Typ::BULK,
            };
        }
        Value::Str(ref s) if s.to_uppercase() == "maxmemory" => {
            return Resp {
                val: Value::Arr([Resp {
                    typ: Typ::BULK,
                    val: Value::Str(s.to_uppercase().to_string()),
                },
                Resp {
                    typ: Typ::BULK,
                    val: Value::Str("0".to_string()),
                }
                ].to_vec()),
                typ: Typ::BULK,
            };
        }
        Value::Str(ref s) if s.to_uppercase() == "timeout" => {
            return Resp {
                val: Value::Arr([Resp {
                    typ: Typ::BULK,
                    val: Value::Str(s.to_uppercase().to_string()),
                },
                Resp {
                    typ: Typ::BULK,
                    val: Value::Str("0".to_string()),
                }
                ].to_vec()),
                typ: Typ::BULK,
            };
        }
        _ => {}         
        
    }
    Resp {
        val: Value::Str("".to_string()),
        typ: Typ::BULK,
    }
}

// Initialize all commands once
fn init_commands() -> HashMap<&'static str, Command> {
    let mut m = HashMap::new();

    m.insert(
        "PING",
        Command {
            func: ping,
            doc: "PING [ARGUMENT] - returns PONG to test server responsiveness.",
            arity: -1,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    m.insert(
        "CONFIG",
        Command {
            func: info,
            doc: "INFO - returns information about the server.",
            arity: 0,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    m
}

// Public accessor for the registry
pub fn commands() -> &'static HashMap<&'static str, Command> {
    COMMANDS.get_or_init(init_commands)
}