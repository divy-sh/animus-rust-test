use std::io::{BufRead, BufReader, Read};
use crate::resp::resp::{Resp, Typ, Value};

pub struct Reader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(rd: R) -> Self {
        Self {
            reader: BufReader::new(rd),
        }
    }

    pub fn read(&mut self) -> std::io::Result<Resp> {
        let mut first_byte = [0u8; 1];
        self.reader.read_exact(&mut first_byte)?;
        match Typ::from_byte(first_byte[0]) {
            Some(Typ::ARRAY) => self.read_array(),
            Some(Typ::BULK) => self.read_bulk(),
            _ => {
                // put back the byte
                self.reader.get_mut().by_ref().take(1).read_exact(&mut first_byte)?;
                self.read_inline()
            }
        }
    }

    fn read_array(&mut self) -> std::io::Result<Resp> {
        let len = self.read_int()?;
        let mut arr = Vec::with_capacity(len as usize);
        for _ in 0..len {
            arr.push(self.read()?);
        }
        Ok(Resp {
            typ: Typ::ARRAY,
            val: Value::Arr(arr)
        })
    }

    fn read_bulk(&mut self) -> std::io::Result<Resp> {
        let len = self.read_int()?;
        if len < 0 {
            return Ok(Resp {
                typ: Typ::BULK,
                val: Value::Str("".to_string())
            });
        }
        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf)?;
        self.read_line()?;
        Ok(Resp {
            typ: Typ::BULK,
            val: Value::Str(String::from_utf8(buf).unwrap_or_default()),
        })
    }

    fn read_inline(&mut self) -> std::io::Result<Resp> {
        let line = self.read_line()?;
        let s = String::from_utf8(line).unwrap_or_default();
        println!("read inline {}", s);
        Ok(Resp {
            typ: Typ::STRING,
            val: Value::Str(s),
        })
    }

    fn read_int(&mut self) -> std::io::Result<i64> {
        let line = self.read_line()?;
        let s = String::from_utf8(line).unwrap_or_default();
        Ok(s.trim().parse().unwrap_or(0))
    }

    fn read_line(&mut self) -> std::io::Result<Vec<u8>> {
        let mut line = Vec::new();
        self.reader.read_until(b'\n', &mut line)?;
        if line.len() >= 2 && line[line.len() - 2] == b'\r' {
            line.truncate(line.len() - 2);
        }
        Ok(line)
    }
}