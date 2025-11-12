use crate::server::{HttpRequest,HttpResponse,StatusCode};
use std::{collections::HashMap, io::Write, net::{TcpListener,TcpStream}};
use std::io::Read;
/// Type alias for a function that handles HTTP requests and returns responses.
/// 
/// Handlers take an `HttpRequest` and return an `HttpResponse`.
/// These functions are stored in the `App` router for specific paths.
pub type Handler = Box<dyn Fn(HttpRequest) -> HttpResponse>;

/// The main application struct that handles HTTP routing and request processing.
/// 
/// An `App` instance maintains a collection of route handlers and processes incoming requests
/// by matching the request URI to registered handlers.
/// 
/// # Examples
/// 
/// ```
/// use mhttp::{App, HttpRequest, HttpResponse, StatusCode};
/// 
/// let mut app = App::new();
/// 
/// app.add_handler("/hello".to_string(), |_| {
///     "Hello, World!".to_string().into_response()
/// });
/// 
/// let req = HttpRequest {
///     method: "GET".to_string(),
///     uri: "/hello".to_string(),
///     version: "HTTP/1.1".to_string(),
///     headers: std::collections::HashMap::new(),
///     body: String::new(),
/// };
/// 
/// let response = app.handle_request(req);
/// assert_eq!(response.status, StatusCode::Ok);
/// ```
pub struct App {
    /// A map of URI paths to their corresponding handler functions
    pub handlers: HashMap<String, Handler>,
}

impl App {
    /// Creates a new empty application instance.
    /// 
    /// The returned `App` has no registered handlers and will return 404 responses for all requests.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use your_crate::App;
    /// 
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a handler function for a specific URI path.
    /// 
    /// When a request with the given URI is received, the provided handler function will be called
    /// to generate the response.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The URI path to register the handler for
    /// * `handler` - A function that takes an `HttpRequest` and returns an `HttpResponse`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mhttp::{App, HttpRequest, HttpResponse};
    /// 
    /// let mut app = App::new();
    /// 
    /// app.add_handler("/api/users".to_string(), |req| {
    ///     format!("Received {} request", req.method).into_response()
    /// });
    /// ```
    pub fn add_handler<F>(&mut self, path: String, handler: F)
    where
        F: Fn(HttpRequest) -> HttpResponse + 'static,
    {
        self.handlers.insert(path, Box::new(handler));
    }

    /// Processes an incoming HTTP request and returns the appropriate response.
    /// 
    /// Looks up the request URI in the registered handlers and calls the corresponding handler.
    /// If no handler is found, returns a 404 Not Found response.
    /// 
    /// # Arguments
    /// 
    /// * `req` - The incoming HTTP request to process
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mhttp::{App, HttpRequest, HttpResponse, StatusCode};
    /// 
    /// let mut app = App::new();
    /// 
    /// app.add_handler("/test".to_string(), |_| {
    ///     "Test response".to_string().into_response()
    /// });
    /// 
    /// let req = HttpRequest {
    ///     method: "GET".to_string(),
    ///     uri: "/test".to_string(),
    ///     version: "HTTP/1.1".to_string(),
    ///     headers: std::collections::HashMap::new(),
    ///     body: String::new(),
    /// };
    /// 
    /// let response = app.handle_request(req);
    /// assert_eq!(response.body, "Test response");
    /// ```
    pub fn handle_request(&self, req: HttpRequest) -> HttpResponse {
        match self.handlers.get(&req.uri.to_string()) {
            Some(handler) => handler(req),
            None => HttpResponse::new(StatusCode::NotFound, "Not Found".to_string()),
        }
    }

    /// Starts a server on a specified port
    /// 
    /// 
    /// 
    /// 
    /// # Arguments
    /// 
    /// 
    /// * `port` - a designated port number
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mhttp::App;
    /// use mhttp::Respondable;
    /// 
    /// fn main() {
    ///     
    ///     let mut app = App::new();
    ///     
    ///     app.add_handler("/".to_string(), |_| => {
    ///         "hello world".into_response()
    ///     });
    /// 
    ///     app.listen(3000);
    /// }
    /// ```
    /// 
    pub fn listen(&self,port: u16) {
        if let Ok(listener) = TcpListener::bind(format!("localhost:{port}")) {
            println!("Listening on a port {port}...");
            loop {
                let (mut socket,_) = listener.accept().unwrap();
                self.process(&mut socket)
            }
        } else {
            panic!("Error occured");
        }
    }

    fn process(&self, socket: &mut TcpStream) {
        // Read request with a fixed-size buffer
        let mut buffer = [0; 4096];
        let mut request_data = Vec::new();
        
        // Read data until we find the end of headers (\r\n\r\n)
        let mut bytes_read = 0;
        let mut headers_end_pos = None;
        
        loop {
            match socket.read(&mut buffer) {
                Ok(0) => break, // Connection closed by client
                Ok(n) => {
                    request_data.extend_from_slice(&buffer[..n]);
                    bytes_read += n;
                    
                    // Look for the end of headers
                    if let Some(pos) = self.find_headers_end(&request_data) {
                        headers_end_pos = Some(pos);
                        break;
                    }
                    
                    // Prevent reading too much data (optional safety measure)
                    if bytes_read >= 4096 {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {:?}", e);
                    return;
                }
            }
        }
        
        if let Some(headers_end_pos) = headers_end_pos {
            // Convert the request data to string
            if let Ok(request_str) = std::str::from_utf8(&request_data[..headers_end_pos + 4]) {
                match HttpRequest::parse(request_str) {
                    Ok(request) => {
                        let response = self.handle_request(request);
                        socket.write_all(response.to_string().as_bytes()).unwrap();
                        socket.flush().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error parsing request: {:?}", e);
                    }
                }
            } else {
                eprintln!("Invalid UTF-8 in request");
            }
        } else {
            eprintln!("No complete HTTP request found");
        }
    }

    // Helper function to find the end of HTTP headers (\r\n\r\n)
    fn find_headers_end(&self,data: &[u8]) -> Option<usize> {
    for i in 0..data.len().saturating_sub(3) {
        if data[i] == b'\r' && 
           data[i + 1] == b'\n' && 
           data[i + 2] == b'\r' && 
           data[i + 3] == b'\n' {
            return Some(i);
        }
    }
    None
}
}



