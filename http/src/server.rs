use std::collections::HashMap;



/// Represents an incoming HTTP request from a client.
/// 
/// This struct contains all the information from an HTTP request including the method,
/// URI, headers, and body. It's passed to handler functions for processing.
/// 
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method:  String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub content_length: usize,
    pub body: String,
}

const SP: char = ' ';
const CR: char = '\r';
const LF: char = '\n';

#[derive(Debug)]
pub enum ParseError {
    InvalidMethod(String),
    InvalidUri(String),
    InvalidVersion(String),
    MalformedRequest,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidMethod(m) => write!(f, "Invalid HTTP method: {}", m),
            ParseError::InvalidUri(u) => write!(f, "Invalid URI: {}", u),
            ParseError::InvalidVersion(v) => write!(f, "Invalid HTTP version: {}", v),
            ParseError::MalformedRequest => write!(f, "Malformed HTTP request"),
        }
    }
}

impl std::error::Error for ParseError {}

impl HttpRequest {
    /// Parses an HTTP request from a string slice.
    /// Assumes request is in the format:
    /// METHOD URI VERSION\r\n
    /// Header: Value\r\n
    /// \r\n
    /// [body]
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let mut lines = input.lines();

        // Parse request line (first line)
        let request_line = lines.next().ok_or(ParseError::MalformedRequest)?;
        let (method, uri, version) = Self::parse_request_line(request_line)?;

        // Parse headers
        let mut headers = HashMap::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }
            let colon_pos = line.find(':').ok_or(ParseError::MalformedRequest)?;
            let header_name = line[..colon_pos].trim().to_string();
            let header_value = line[colon_pos + 1..].trim().to_string();
            headers.insert(header_name, header_value);
        }

        let content_length = headers
            .get("Content-Length")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        // Collect remaining lines as body
        let body = lines.collect::<Vec<_>>().join("\n");

        Ok(HttpRequest {
            method,
            uri,
            version,
            headers,
            content_length,
            body,
        })
    }

    fn parse_request_line(line: &str) -> Result<(String, String, String), ParseError> {
        let mut parts = line.split(SP);
        let method = parts.next().ok_or(ParseError::MalformedRequest)?.to_string();
        let uri = parts.next().ok_or(ParseError::MalformedRequest)?.to_string();
        let version_with_crlf = parts.next().ok_or(ParseError::MalformedRequest)?;

        // Remove \r\n if present
        let version = version_with_crlf.trim_end_matches(|c| c == CR || c == LF).to_string();

        // Validate method
        match method.as_str() {
            "GET" | "POST" | "PUT" | "DELETE" | "OPTIONS" | "HEAD" | "PATCH" => (),
            _ => return Err(ParseError::InvalidMethod(method.to_string())),
        }

        // Validate version
        match version.as_str() {
            "HTTP/1.0" | "HTTP/1.1" => (),
            _ => return Err(ParseError::InvalidVersion(version.to_string())),
        }

        // Basic URI validation (no regex)
        if !Self::is_valid_uri(&uri) {
            return Err(ParseError::InvalidUri(uri));
        }

        Ok((method, uri, version))
    }

    fn is_valid_uri(uri: &String) -> bool {
        // Basic checks: starts with /, no control chars, etc.
        if !uri.starts_with('/') {
            return false;
        }
        // Check for control characters (0x00-0x1F) and DEL (0x7F)
        if uri.chars().any(|c| c.is_control()) {
            return false;
        }
        // Additional checks can be added here
        true
    }
}



/// Represents an HTTP status code.
/// 
/// This enum provides type-safe access to common HTTP status codes.
/// Each variant corresponds to a specific status code with its associated meaning.
/// 
/// # Examples
/// 
/// ```
/// use your_crate::StatusCode;
/// 
/// let ok = StatusCode::Ok;
/// assert_eq!(ok.as_u16(), 200);
/// assert_eq!(ok.reason_phrase(), "OK");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusCode {
    /// 200 OK - Standard response for successful HTTP requests
    Ok = 200,
    /// 404 Not Found - The requested resource could not be found
    NotFound = 404,
    /// 400 Bad Request - The server cannot or will not process the request due to an apparent client error
    BadRequest = 400,
    /// 500 Internal Server Error - A generic error message when the server encounters an unexpected condition
    InternalServerError = 500,
    // Add more as needed
}

impl StatusCode {
    /// Returns the numeric value of the status code.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use your_crate::StatusCode;
    /// 
    /// assert_eq!(StatusCode::Ok.as_u16(), 200);
    /// ```
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    /// Returns the reason phrase associated with the status code.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use your_crate::StatusCode;
    /// 
    /// assert_eq!(StatusCode::Ok.reason_phrase(), "OK");
    /// ```
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

/// Represents an HTTP response that can be sent back to the client.
/// 
/// An `HttpResponse` contains the status code, headers, and body of the response.
/// It provides methods to create responses and convert them to the HTTP wire format.
/// 
/// # Examples
/// 
/// ```
/// use your_crate::{HttpResponse, StatusCode};
/// 
/// let response = HttpResponse::new(StatusCode::Ok, "Hello, World!".to_string());
/// println!("{}", response.to_string());
/// ```
#[derive(Clone, Debug)]
pub struct HttpResponse {
    /// The HTTP version (e.g., "HTTP/1.1")
    pub version: String,
    /// The HTTP status code
    pub status: StatusCode,
    /// The status message (e.g., "OK", "Not Found")
    pub status_message: String,
    /// HTTP headers as key-value pairs
    pub headers: HashMap<String, String>,
    /// The response body
    pub body: String,
}

impl HttpResponse {
    /// Creates a new HTTP response with the given status and body.
    /// 
    /// This method automatically sets the `Content-Length` header based on the body length.
    /// The version defaults to "HTTP/1.1".
    /// 
    /// # Arguments
    /// 
    /// * `status` - The HTTP status code for the response
    /// * `body` - The response body as a string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use your_crate::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::new(StatusCode::Ok, "Hello".to_string());
    /// assert_eq!(response.status, StatusCode::Ok);
    /// assert_eq!(response.body, "Hello");
    /// ```
    pub fn new(status: StatusCode, body: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        
        Self {
            version: "HTTP/1.1".to_string(),
            status,
            status_message: status.reason_phrase().to_string(),
            headers,
            body,
        }
    }

    /// Converts the response to its HTTP string representation for sending over the network.
    /// 
    /// The format follows the HTTP/1.1 specification with headers separated from the body by `\r\n\r\n`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use your_crate::{HttpResponse, StatusCode};
    /// 
    /// let response = HttpResponse::new(StatusCode::Ok, "Hello".to_string());
    /// let http_string = response.to_string();
    /// assert!(http_string.starts_with("HTTP/1.1 200 OK"));
    /// ```
    pub fn to_string(&self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.version,
            self.status.as_u16(),
            self.status_message
        );

        // Add headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n"); // End of headers
        response.push_str(&self.body);

        response
    }
}

impl Default for HttpResponse {
    /// Creates a default HTTP response with status 200 OK and an empty body.
    /// 
    /// This is equivalent to `HttpResponse::new(StatusCode::Ok, String::new())`.
    fn default() -> Self {
        Self::new(StatusCode::Ok, String::new())
    }
}

/// A trait for types that can be converted into an HTTP response.
/// 
/// This trait allows different types to be used as response bodies in handlers.
/// Common implementations exist for `String`, `&str`, `HttpResponse`, and `Result<T, E>`.
/// 
/// # Examples
/// 
/// ```
/// use your_crate::{Respondable, HttpResponse, StatusCode};
/// 
/// let response: HttpResponse = "Hello".into_response();
/// assert_eq!(response.status, StatusCode::Ok);
/// 
/// let response: HttpResponse = "Hello".to_string().into_response();
/// assert_eq!(response.status, StatusCode::Ok);
/// ```
pub trait Respondable {
    /// Converts the implementing type into an HTTP response.
    /// 
    /// The default implementation for most types returns a 200 OK response with the value as the body.
    fn into_response(self) -> HttpResponse;
}

/// Implements `Respondable` for `String`.
/// 
/// Creates a response with status 200 OK and the string as the body.
impl Respondable for String {
    fn into_response(self) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, self)
    }
}

/// Implements `Respondable` for `&str`.
/// 
/// Creates a response with status 200 OK and the string slice as the body.
impl Respondable for &str {
    fn into_response(self) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, self.to_string())
    }
}

/// Implements `Respondable` for `HttpResponse`.
/// 
/// Returns the response unchanged, allowing existing responses to be returned directly.
impl Respondable for HttpResponse {
    fn into_response(self) -> HttpResponse {
        self
    }
}

/// Implements `Respondable` for `Result<T, String>`.
/// 
/// On success (`Ok`), converts the inner value to a response.
/// On error (`Err`), creates a 500 Internal Server Error response with the error message as the body.
impl<T> Respondable for Result<T, String>
where
    T: Respondable,
{
    fn into_response(self) -> HttpResponse {
        match self {
            Ok(value) => value.into_response(),
            Err(error_msg) => HttpResponse::new(StatusCode::InternalServerError, error_msg),
        }
    }
}


/// For convenience, `Unit` type also implements Respondable.
/// This allows no return in a handler.
impl Respondable for () {
    fn into_response(self) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok,"".to_string())
    }
}

