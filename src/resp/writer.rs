use crate::resp::resp::{Resp, Value};
use std::io::{self, Write};

pub struct Writer<W: Write> {
    writer: W,
}

impl<W: Write> Writer<W> {
    pub fn new(writer: W) -> Self {
        Writer { writer }
    }

    pub fn write(&mut self, v: Resp) -> io::Result<()> {
        let bytes = v.marshal();
        let _err: Result<(), io::Error> = self.writer.write_all(&bytes);
        self.writer.flush()?;
        Ok(())
    }
}

impl Resp {
    pub fn marshal(&self) -> Vec<u8> {
        match &self.val {
            Value::Arr(arr) => self.marshal_array(arr),
            Value::Str(s) => self.marshal_string(s),
            Value::Num(n) => self.marshal_int(*n),
        }
    }

    fn marshal_int(&self, num: i64) -> Vec<u8> {
        let mut bytes = vec![b':'];
        bytes.extend_from_slice(num.to_string().as_bytes());
        bytes.extend_from_slice(b"\r\n");
        bytes
    }

    fn marshal_string(&self, s: &str) -> Vec<u8> {
        let mut bytes = vec![b'+'];
        bytes.extend_from_slice(s.as_bytes());
        bytes.extend_from_slice(b"\r\n");
        bytes
    }

    fn marshal_array(&self, arr: &[Resp]) -> Vec<u8> {
        let mut bytes = vec![b'*'];
        bytes.extend_from_slice(arr.len().to_string().as_bytes());
        bytes.extend_from_slice(b"\r\n");

        for item in arr {
            bytes.extend_from_slice(&item.marshal());
        }

        bytes
    }

    // Optional: add null and error if you want
    pub fn marshal_null() -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }

    pub fn marshal_error(s: &str) -> Vec<u8> {
        let mut bytes = vec![b'-'];
        bytes.extend_from_slice(s.as_bytes());
        bytes.extend_from_slice(b"\r\n");
        bytes
    }
}
