use failure::Error;
use failure_derive::Fail;
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::env::{self, VarError};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

pub const AOC_SESSION_ENV: &str = "AOC_SESSION";

pub fn get_session_token() -> Result<String, VarError> {
    env::var(AOC_SESSION_ENV)
}

#[derive(Clone, Debug)]
pub struct Client {
    session: String,
}

impl Client {
    pub fn new<S: Into<String>>(session: S) -> Client {
        Client {
            session: session.into(),
        }
    }

    pub fn from_env() -> Result<Client, Error> {
        let session = get_session_token()?;
        Ok(Client::new(session))
    }

    pub fn get_input(&self, year: i32, day: i32) -> Result<String, Error> {
        let connector = TlsConnector::new()?;
        let stream = TcpStream::connect("adventofcode.com:443")?;
        let mut stream = connector.connect("adventofcode.com", stream)?;

        let nl = "\r\n";
        write!(stream, "GET /{}/day/{}/input HTTP/1.1{}", year, day, nl)?;
        write!(stream, "Host: adventofcode.com{}", nl)?;
        write!(stream, "Accept: text/plain{}", nl)?;
        write!(stream, "Connection: close{}", nl)?;
        write!(stream, "Cookie: session={}{}", self.session, nl)?;
        write!(stream, "User-Agent: aoc-rs/0.1.0{}", nl)?;
        write!(stream, "{}{}", nl, nl)?;

        let mut reader = BufReader::new(stream);
        let mut buf = String::new();

        macro_rules! read_line {
            () => {{
                buf.clear();
                let bytes = reader.read_line(&mut buf)?;
                if bytes == 0 {
                    return Err(HttpError::UnexpectedEndOfStream.into());
                }
                buf.trim()
            }};
        }

        let status: i32 = {
            let line = read_line!();
            let code = match line.split(' ').nth(1) {
                Some(s) => s,
                None => return Err(HttpError::InvalidStatusLine.into()),
            };
            code.parse()?
        };

        if status != 200 {
            return Err(HttpError::StatusCode(status).into());
        }

        let headers = {
            let mut set = HashMap::<String, String>::new();
            loop {
                let line = read_line!();
                if line.is_empty() {
                    break; // end of headers
                }

                let mut pair = line.splitn(2, ':');
                let (key, value) = match (pair.next(), pair.next()) {
                    (Some(k), Some(v)) => (k, v),
                    _ => return Err(HttpError::InvalidHeader.into()),
                };

                set.insert(key.trim().to_owned(), value.trim().to_owned());
            }
            set
        };

        let content_length: usize = {
            headers
                .get("Content-Length")
                .map(|s| &**s)
                .unwrap_or("0")
                .parse()?
        };

        let input = {
            let mut input_buf = vec![0u8; content_length];
            reader.read_exact(&mut input_buf)?;
            String::from_utf8(input_buf)?
        };

        Ok(input)
    }
}

#[derive(Debug, Fail)]
pub enum HttpError {
    #[fail(display = "unexpected end of stream")]
    UnexpectedEndOfStream,
    #[fail(display = "invalid HTTP status line format")]
    InvalidStatusLine,
    #[fail(display = "invalid HTTP header format")]
    InvalidHeader,
    #[fail(display = "HTTP error code {}", _0)]
    StatusCode(i32),
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn test_client() {
        let client = match Client::from_env() {
            Ok(client) => client,
            Err(_) => return, // skip test for non-configured systems
        };
        let input = client.get_input(2018, 1).unwrap();
        assert!(!input.is_empty());
    }
}
