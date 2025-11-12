#[derive(Debug, Clone)]
pub enum Typ {
	STRING,
	ERROR,
	INTEGER,
	BULK,
	ARRAY
}

impl Typ {
    pub fn from_byte(byte: u8) -> Option<Typ> {
        match byte {
            b'+' => Some(Typ::STRING),
            b'-' => Some(Typ::ERROR),
            b':' => Some(Typ::INTEGER),
            b'$' => Some(Typ::BULK),
            b'*' => Some(Typ::ARRAY),
            _ => None,
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            Typ::STRING => b'+',
            Typ::ERROR => b'-',
            Typ::INTEGER => b':',
            Typ::BULK => b'$',
            Typ::ARRAY => b'*',
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(i64),
    Str(String),
    Arr(Vec<Resp>),
}

#[derive(Debug, Clone)]
pub struct Resp {
    pub typ: Typ,
    pub val: Value
}