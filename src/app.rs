use crate::server::{HttpRequest,HttpResponse,StatusCode};
use std::collections::HashMap;

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
/// use your_crate::{App, HttpRequest, HttpResponse, StatusCode};
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
    /// use your_crate::{App, HttpRequest, HttpResponse};
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
    /// use your_crate::{App, HttpRequest, HttpResponse, StatusCode};
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
}