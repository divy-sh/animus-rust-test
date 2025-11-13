use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use animus_rust::commands::handler;
use animus_rust::resp::{reader, resp, writer};

fn main() {
    handle();
}

// Retry helper
fn retry<F, T, E>(max_retries: usize, delay: Duration, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for _i in 0..max_retries {
        match f() {
            Ok(val) => return Ok(val),
            Err(_e) => {
                thread::sleep(delay);
            }
        }
    }
    f()
}

fn handle() {
    let listener = retry(5, Duration::from_secs(2), || {
        TcpListener::bind("0.0.0.0:6379")
    })
    .expect("Failed to start server after retries");
    println!("Listening to port: 6379...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_requests(stream));
            }
            Err(_e) => {
                // Could implement retry on accept here if desired
            }
        }
    }
}

fn handle_requests(stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let writer = BufWriter::new(&stream);
    let mut reader = reader::Reader::new(reader);
    let mut writer = writer::Writer::new(writer);

    loop {
        let r = match reader.read() {
            Ok(r) => r,
            Err(_e) => {
                // println!("error in reading data {:?}", e);
                return;
            } // Connection closed or read error
        };
        // Extract the array of arguments
        let args = match &r.val {
            resp::Value::Arr(a) => a,
            _ => {
                let _ = writer.write(resp::Resp {
                    typ: resp::Typ::STRING,
                    val: resp::Value::Str("Invalid request".to_string()),
                });
                continue;
            }
        };

        if args.is_empty() {
            let _ = writer.write(resp::Resp {
                typ: resp::Typ::STRING,
                val: resp::Value::Str("Invalid request".to_string()),
            });
            continue;
        }

        // Extract the command as a string
        let cmd = match &args[0].val {
            resp::Value::Str(s) => s.to_uppercase(),
            _ => {
                let _ = writer.write(resp::Resp {
                    typ: resp::Typ::STRING,
                    val: resp::Value::Str("Invalid command".to_string()),
                });
                continue;
            }
        };

        let cmd_args = &args[1..];

        if cmd == "QUIT" {
            let _ = writer.write(resp::Resp {
                typ: resp::Typ::STRING,
                val: resp::Value::Str("OK".to_string()),
            });
            return;
        }

        match handler::commands().get(cmd.as_str()) {
            Some(handler) => {
                let result = (handler.func)(cmd_args.to_vec());
                let _ = writer.write(result);
            }
            None => {
                let _ = writer.write(resp::Resp {
                    typ: resp::Typ::STRING,
                    val: resp::Value::Str("Invalid command".to_string()),
                });
            }
        }
    }
}
