use std::io::Write;
use std::sync::Arc;

use crate::constants::{headers, mime_types};
use crate::errors::{AppError, ClientError::BadRequest};
use crate::http::{Request, Response};
use crate::utils::{get_header_value, get_path_parts, read_file, write_file};
use crate::Result;

pub fn handle_empty<T: Write>(s: &mut T) -> Result<()> {
    s.write_all(&Response::Ok(None).to_vec())?;
    Ok(())
}

pub fn handle_echo<T: Write>(s: &mut T, r: &Request) -> Result<()> {
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    s.write_all(
        &Response::Ok(Some((&r.body, mime_types::PLAIN_TEXT.to_owned(), encoding))).to_vec(),
    )?;
    Ok(())
}

pub fn handle_user_agent<T: Write>(s: &mut T, r: &Request) -> Result<()> {
    let body = get_header_value(headers::USER_AGENT, &r.headers);
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    if let Some(b) = body {
        let response = Response::Ok(Some((
            b.as_bytes(),
            mime_types::PLAIN_TEXT.to_owned(),
            encoding,
        )));
        s.write_all(&response.to_vec())?;
    } else {
        return Err(BadRequest.into());
    }
    Ok(())
}

pub fn handle_get_file<T: Write>(s: &mut T, r: &Request, fp: Arc<String>) -> Result<()> {
    let filename = get_path_parts(&r.path)[1];
    let contents = read_file(fp, filename)?;
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    let response = Response::Ok(Some((
        &contents,
        mime_types::OCTET_STREAM.to_owned(),
        encoding,
    )));
    s.write_all(&response.to_vec())?;
    Ok(())
}

pub fn handle_post_file<T: Write>(s: &mut T, r: &Request, fp: Arc<String>) -> Result<()> {
    let filename = get_path_parts(&r.path)[1];
    if !r.body.is_empty() {
        write_file(fp, filename, r)?;
    } else {
        return Err(BadRequest.into());
    };
    let response = Response::Created;
    s.write_all(&response.to_vec())?;
    Ok(())
}

pub fn handle_error<T: Write>(s: &mut T, err: AppError) -> Result<()> {
    match err {
        AppError::Server(e) => s.write_all(&Response::ServerError(e).to_vec())?,
        AppError::Client(e) => s.write_all(&Response::ClientError(e).to_vec())?,
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::handle_empty as empty;
    use crate::http::Response;
    use std::io::Write;

    struct MockStream {
        write_all_bytes: Vec<u8>,
    }

    impl MockStream {
        fn new() -> Self {
            Self {
                write_all_bytes: vec![],
            }
        }
    }
    impl Write for MockStream {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            unimplemented!()
        }
        fn flush(&mut self) -> std::io::Result<()> {
            unimplemented!()
        }
        fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
            self.write_all_bytes.extend(buf);
            Ok(())
        }
    }
    #[test]
    fn handle_empty() {
        let mut stream = MockStream::new();
        let _ = empty(&mut stream);
        assert_eq!(stream.write_all_bytes, Response::Ok(None).to_vec());
    }
}
