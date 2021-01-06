//
use std::str;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::Utf8Error,
};

use super::QueryString;

use std::error::Error;

// super here means go one level up
use super::method::{Method, MethodError};

// option is always automatically imported
// specify a lifetime - signify that it is the lifetime of the buffer
// provide a default implementation of the Debug trait, all fields must also
// have implementation
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        // Converts from `&Option<T>` to `Option<&T>`.
        // we are interested in the query string and we return a reference to it
        // in an option
        self.query_string.as_ref()
    }
}

// also implements try_into<Request>
impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {
    type Error = ParseError;

    // or Result<Request<'buf>, Self::Error>
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // ways of writing the same

        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => {return Err(ParseError::InvalidEncoding)},
        // }

        // match str::from_utf8((buf)).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => {return Err(e)},
        // }

        // let request =
        // str::from_utf8((buf)).or(Err(ParseError::InvalidEncoding))?;

        // ? tries to convert the error it sees in the error clause (Utf8Error)
        // to the target error (ParseError) using From
        let request = str::from_utf8(buf)?;

        // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...

        // variable shadowing - new variable with a name of request
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // implementing FromStr gives us parse for free
        // ? is using From<MethodError>
        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if query_string.is_some() {
        //     // safe because we checked for some
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop {
    //     let c = iter.next();
    //     match c {
    //         Some(_) => {},
    //         None => {break},
    //     }
    // };

    // indices of valid chars, accounting for the fact that special symbols may
    // be represented using more than 1 byte
    for (i, c) in request.char_indices() {
        if c == ' ' || c == '\r' {
            // we know that the space character is only 1 byte long so it is
            // o.k. to skip it by using i + 1
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

// need to implement Debug and Display in order to implement Error for
// ParseError

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
