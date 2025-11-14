use crate::resp::resp::{Resp, Typ, Value};
use std::{collections::HashMap, sync::OnceLock};

pub struct Command {
    pub func: fn(Vec<Resp>) -> Resp,
    pub doc: &'static str,
    pub arity: i32,
    pub flags: &'static [&'static str],
    pub first_key: i32,
    pub last_key: i32,
    pub step: i32,
}

static COMMANDS: OnceLock<HashMap<&'static str, Command>> = OnceLock::new();

// ----------------- Example command handlers -----------------
fn ping(_args: Vec<Resp>) -> Resp {
    Resp {
        val: Value::Str("PONG".to_string()),
        typ: Typ::STRING,
    }
}

fn info(_args: Vec<Resp>) -> Resp {
    Resp {
        val: Value::Str("INFO".to_string()),
        typ: Typ::BULK,
    }
}

// Placeholder for other commands
fn dummy(_args: Vec<Resp>) -> Resp {
    Resp {
        val: Value::Str("OK".to_string()),
        typ: Typ::STRING,
    }
}

// ----------------- Initialize commands -----------------
fn init_commands() -> HashMap<&'static str, Command> {
    let mut m = HashMap::new();

    // Connection
    m.insert(
        "PING",
        Command {
            func: ping,
            doc: r#"PING [ARGUMENT]
Returns PONG to test server responsiveness."#,
            arity: -1,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    m.insert(
        "COMMAND",
        Command {
            func: dummy,
            doc: r#"COMMAND
Returns metadata about all registered commands."#,
            arity: 0,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    m.insert(
        "INFO",
        Command {
            func: info,
            doc: r#"INFO
Returns information and statistics about the server."#,
            arity: 0,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    m.insert(
        "CONFIG",
        Command {
            func: dummy,
            doc: r#"CONFIG
Command to handle server configuration."#,
            arity: -1,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    // Strings
    let string_cmds = [
        (
            "APPEND",
            3,
            r#"APPEND [KEY] [VALUE]
Appends a value to a key and returns the new length of the string."#,
        ),
        (
            "DECR",
            2,
            r#"DECR [KEY]
Decrements the integer value of a key by one."#,
        ),
        (
            "DECRBY",
            3,
            r#"DECRBY [KEY] [DECREMENT]
Decrements the integer value of a key by the given amount."#,
        ),
        (
            "GET",
            2,
            r#"GET [KEY]
Gets the value of a key."#,
        ),
        (
            "GETDEL",
            2,
            r#"GETDEL [KEY]
Gets the value of a key and deletes it."#,
        ),
        (
            "GETEX",
            3,
            r#"GETEX [KEY] [EXPIRATION]
Gets the value of a key and sets an expiration."#,
        ),
        (
            "GETRANGE",
            4,
            r#"GETRANGE [KEY] [START] [END]
Gets a substring of the string stored at a key."#,
        ),
        (
            "GETSET",
            3,
            r#"GETSET [KEY] [VALUE]
Gets the previous key value and then sets it to the passed value."#,
        ),
        (
            "INCR",
            2,
            r#"INCR [KEY]
Increments the integer value of a key by one."#,
        ),
        (
            "INCRBY",
            3,
            r#"INCRBY [KEY] [INCREMENT]
Increments the integer value of a key by the given amount."#,
        ),
        (
            "INCRBYFLOAT",
            3,
            r#"INCRBYFLOAT [KEY] [INCREMENT]
Increments the float value of a key by the given amount."#,
        ),
        (
            "LCS",
            4,
            r#"LCS [KEY1] [KEY2] LEN
Finds the Longest Common Subsequence between the value of two keys.
Send the optional LEN argument to get just the length."#,
        ),
        (
            "MGET",
            -2,
            r#"MGET key [key ...]
Returns the values for all the keys. Returns nil for a non-existing key."#,
        ),
        (
            "MSET",
            -3,
            r#"MSET key value [key1 value1 ...]
Sets the values for all the keys value pair."#,
        ),
        (
            "SET",
            -3,
            r#"SET [KEY] [VALUE]
Sets the value of a key."#,
        ),
        ("SETRANGE", -3, r#"SETRANGE key offset value"#),
        (
            "SETEX",
            4,
            r#"SET [KEY] [VALUE] [EX SECONDS]
Sets the value of a key with expiration in seconds."#,
        ),
        (
            "STRLEN",
            2,
            r#"STRLEN [KEY]
Returns the length of the string value stored at key."#,
        ),
    ];

    for &(name, arity, doc) in &string_cmds {
        m.insert(
            name,
            Command {
                func: dummy,
                doc,
                arity,
                flags: &["readonly", "fast"],
                first_key: 0,
                last_key: 0,
                step: 0,
            },
        );
    }

    // Hashes
    let hash_cmds = [
        (
            "HSET",
            -4,
            r#"HSET [KEY] [FIELD] [VALUE]
Sets a field in the hash stored at key to a value."#,
        ),
        (
            "HGET",
            3,
            r#"HGET [KEY] [FIELD]
Gets the value of a field in the hash stored at key."#,
        ),
        (
            "HEXISTS",
            3,
            r#"HEXISTS [KEY] [FIELD]
Checks if the hash and the field combination exists in the store."#,
        ),
        (
            "HEXPIRE",
            3,
            r#"HEXPIRE key seconds [NX XX GT LT]
Sets a timeout on hash key. After the timeout, the key gets deleted."#,
        ),
        (
            "HDEL",
            3,
            r#"HDEL [KEY] [FIELD]
Deletes a field from the hash stored at key."#,
        ),
        (
            "HGETALL",
            2,
            r#"HGETALL [KEY]
Returns all fields and values of the hash stored at key."#,
        ),
    ];

    for &(name, arity, doc) in &hash_cmds {
        m.insert(
            name,
            Command {
                func: dummy,
                doc,
                arity,
                flags: &["readonly", "fast"],
                first_key: 0,
                last_key: 0,
                step: 0,
            },
        );
    }

    // Lists
    let list_cmds = [
        (
            "RPOP",
            -2,
            r#"RPOP [KEY] [COUNT]
Removes and returns the last element(s) of the list stored at key."#,
        ),
        (
            "RPUSH",
            -3,
            r#"RPUSH [KEY] [VALUE] [VALUE ...]
Inserts one or more elements at the end of the list stored at key."#,
        ),
    ];

    for &(name, arity, doc) in &list_cmds {
        m.insert(
            name,
            Command {
                func: dummy,
                doc,
                arity,
                flags: &[],
                first_key: 0,
                last_key: 0,
                step: 0,
            },
        );
    }

    // Sets
    let set_cmds = [
        (
            "SADD",
            -3,
            r#"SADD [KEY] [MEMBER] [MEMBER ...]
Adds one or more members to the set stored at key."#,
        ),
        (
            "SCARD",
            2,
            r#"SCARD [KEY]
Returns the number of members in the set stored at key."#,
        ),
        (
            "SDIFF",
            -2,
            r#"SDIFF [KEY] [KEY ...]
Returns the members of the set resulting from the difference between the first set and all the successive sets."#,
        ),
        (
            "SISMEMBER",
            3,
            r#"SISMEMBER [KEY] [MEMBER]
Returns if member is a member of the set stored at key."#,
        ),
    ];

    for &(name, arity, doc) in &set_cmds {
        m.insert(
            name,
            Command {
                func: dummy,
                doc,
                arity,
                flags: &["readonly", "fast"],
                first_key: 0,
                last_key: 0,
                step: 0,
            },
        );
    }

    // Help
    m.insert(
        "HELP",
        Command {
            func: dummy,
            doc: r#"HELP [COMMAND]
Provides details on how to use a command and what the command actually does."#,
            arity: -1,
            flags: &["readonly", "fast"],
            first_key: 0,
            last_key: 0,
            step: 0,
        },
    );

    // Generic commands
    let generic_cmds = [
        (
            "COPY",
            3,
            r#"COPY [key1] [key2]
Copies value(s) of key1 into key2. If key2 doesn't exist, creates key2 and copies the value of key1 into key2."#,
        ),
        (
            "DEL",
            -2,
            r#"DEL key1 [keys...]
Deletes all the keys passed as argument. Ignores the keys in the argument that don't exist."#,
        ),
        (
            "EXISTS",
            -2,
            r#"EXISTS key1 [keys...]
Returns an integer denoting how many of the passed keys exist in the cache."#,
        ),
        (
            "EXPIRE",
            3,
            r#"EXPIRE key seconds [NX XX GT LT]
Sets a timeout on key. After the timeout, the key gets deleted."#,
        ),
        (
            "EXPIREAT",
            3,
            r#"EXPIREAT key unix-time-seconds [NX XX GT LT]
Sets the timeout of a key to the unix time stamp in seconds. After the timeout, the key gets deleted."#,
        ),
        (
            "EXPIRETIME",
            2,
            r#"EXPIRETIME key
Returns the expire time of a key in unix epoch seconds. -1 if the key doesn't have an expiry set, -2 if the key doesn't exist."#,
        ),
        (
            "KEYS",
            1,
            r#"KEYS
Returns the keys that exist in the store."#,
        ),
    ];

    for &(name, arity, doc) in &generic_cmds {
        m.insert(
            name,
            Command {
                func: dummy,
                doc,
                arity,
                flags: &["readonly", "fast"],
                first_key: 0,
                last_key: 0,
                step: 0,
            },
        );
    }

    m
}

// ----------------- Public accessor -----------------
pub fn commands() -> &'static HashMap<&'static str, Command> {
    COMMANDS.get_or_init(init_commands)
}
