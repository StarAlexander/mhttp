use crate::server::{HttpRequest,HttpResponse,StatusCode};
use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}};
/// Type alias for a function that handles HTTP requests and returns responses.
/// 
/// Handlers take an `HttpRequest` and return an `HttpResponse`.
/// These functions are stored in the `App` router for specific paths.
pub type Handler = Box<dyn Fn(HttpRequest) -> HttpResponse>;

pub type Middleware = Box<dyn Fn(&HttpRequest) -> ()>;

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
/// app.get("/hello".to_string(), |_| {
///     "Hello, World!".to_string().into_response()
/// });
/// 
/// app.listen(3000);
/// ```
pub struct App {
    pub handlers: HashMap<String, HashMap<String,Handler>>,
    pub middlewares: Vec<Middleware>,
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
            middlewares: Vec::new(),
        }
    }

    /// Registers a handler function for a `GET` request to a specific path.
    /// 
    /// When a `GET` request with the given URI is received, the provided handler will be
    /// called to generate the response.
    /// 
    /// If a handler on this route exists, it will be rewritten.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The URI path to register the handler for
    /// * `handler` - A function that takes an `HttpRequest` and return an `HttpResponse`
    /// 
    /// 
    /// # Examples
    /// 
    /// 
    /// ```
    /// use mhttp::{App, Respondable};
    /// 
    /// fn main() {
    ///     let mut app = App::new();
    ///     
    ///     app.get(String::from("/"), |req| {
    ///         "Hello World!".into_response()    
    ///     })
    /// }
    /// ```
    pub fn get<F>(&mut self, path: String, handler: F)
    where 
        F: Fn(HttpRequest) -> HttpResponse + 'static,
         {
            let get =String::from("GET");
            let method_map = self.handlers.entry(path).or_insert(HashMap::new());

            // rewrite if exists
            if method_map.contains_key(&get) {
                method_map.remove(&get);
            }
            method_map.insert(get,Box::new(handler));
         }
    


    /// Registers a handler function for a `POST` request to a specific path.
    /// 
    /// When a `POST` request with the given URI is received, the provided handler will be
    /// called to generate the response.
    /// 
    /// If a handler on this route exists, it will be rewritten.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The URI path to register the handler for
    /// * `handler` - A function that takes an `HttpRequest` and return an `HttpResponse`
    /// 
    /// 
    /// # Examples
    /// 
    /// 
    /// ```
    /// use mhttp::{App, Respondable};
    /// 
    /// fn main() {
    ///     let mut app = App::new();
    ///     
    ///     app.post(String::from("/"), |req| {
    ///         "Hello World!".into_response()    
    ///     })
    /// }
    /// ```
    pub fn post<F>(&mut self, path: String, handler: F)
    where 
        F: Fn(HttpRequest) -> HttpResponse + 'static,
         {
            let post =String::from("POST");
            let method_map = self.handlers.entry(path).or_insert(HashMap::new());
            if method_map.contains_key(&post) {
                method_map.remove(&post);
            }
            method_map.insert(post,Box::new(handler));
         }
    


    /// Registers a handler function for a `PUT` request to a specific path.
    /// 
    /// When a `PUT` request with the given URI is received, the provided handler will be
    /// called to generate the response.
    /// 
    /// If a handler on this route exists, it will be rewritten.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The URI path to register the handler for
    /// * `handler` - A function that takes an `HttpRequest` and return an `HttpResponse`
    /// 
    /// 
    /// # Examples
    /// 
    /// 
    /// ```
    /// use mhttp::{App, Respondable};
    /// 
    /// fn main() {
    ///     let mut app = App::new();
    ///     
    ///     app.put(String::from("/"), |req| {
    ///         "Hello World!".into_response()    
    ///     })
    /// }
    /// ```
    pub fn put<F>(&mut self, path: String, handler: F)
    where 
        F: Fn(HttpRequest) -> HttpResponse + 'static,
         {
            let put =String::from("PUT");
            let method_map = self.handlers.entry(path).or_insert(HashMap::new());
            if method_map.contains_key(&put) {
                method_map.remove(&put);
            }
            method_map.insert(put,Box::new(handler));
         }
    



    /// Registers a handler function for a `DELETE` request to a specific path.
    /// 
    /// When a `DELETE` request with the given URI is received, the provided handler will be
    /// called to generate the response.
    /// 
    /// If a handler on this route exists, it will be rewritten.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The URI path to register the handler for
    /// * `handler` - A function that takes an `HttpRequest` and return an `HttpResponse`
    /// 
    /// 
    /// # Examples
    /// 
    /// 
    /// ```
    /// use mhttp::{App, Respondable};
    /// 
    /// fn main() {
    ///     let mut app = App::new();
    ///     
    ///     app.delete(String::from("/"), |req| {
    ///         "Hello World!".into_response()    
    ///     })
    /// }
    /// ```
    pub fn delete<F>(&mut self, path: String, handler: F)
    where 
        F: Fn(HttpRequest) -> HttpResponse + 'static,
         {
            let delete =String::from("DELETE");
            let method_map = self.handlers.entry(path).or_insert(HashMap::new());
            if method_map.contains_key(&delete) {
                method_map.remove(&delete);
            }
            method_map.insert(delete,Box::new(handler));
         }
    

    
    /// Serving static files in a specified directory.
    /// 
    /// 
    /// # Arguments
    /// 
    /// * `dir_path` - a path to the directory containing files to be served.
    /// 
    /// 
    /// * `serving_path` - a prefix path to the files of the directory.
    /// Pass "" to default to "/".
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mhttp::{App};
    /// 
    /// 
    /// fn main() {
    /// 
    ///     let mut app = App::new();
    /// 
    ///     app.serve_static_dir(String::from("/static"),String::from("")); // Assuming there is a "/static" directory at the root of the project.
    /// 
    /// 
    ///     app.listen(3000);
    /// }
    /// ```
    pub fn serve_static_dir(&mut self, dir_path: String, serving_path: String) {
    if let Ok(dir) = std::fs::read_dir(&dir_path) {
        for file_entry in dir {
            let file_entry = match file_entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error reading directory entry: {}", e);
                    continue;
                }
            };
            
            let file_name = file_entry.file_name();
            let file_path = file_entry.path();


            if file_path.is_file() {
                let file_content = match std::fs::read_to_string(&file_path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error reading file {:?}: {}", file_path, e);
                        continue;
                    }
                };
                let route_path = format!("{}/{}", serving_path,file_name.to_string_lossy());
                self.get(route_path, move |_| {
                    let cloned_content = file_content.clone();
                    HttpResponse::new(StatusCode::Ok, String::from(cloned_content)) 
                });
            } else {
                self.serve_static_dir(file_path.to_string_lossy().to_string(), format!("{}/{}",serving_path,file_name.to_string_lossy()));
            }
        }
    } else {
        panic!("Error happened.");
    }
}

    /// Adds a middleware function 
    /// 
    /// Multiple middlewares are executed in a sequential order, corresponding
    /// to the order in which they were defined in your code, that is:
    /// 
    /// ```
    /// app.use_middleware(|_| {
    /// 
    ///     println!("First!");
    ///     
    /// });
    /// 
    /// app.use_middleware(|_| {
    ///     
    ///     println!("Second!");
    /// })
    /// ```
    /// 
    /// The result will be:
    /// 
    /// ```
    /// "First!"
    /// "Second!"
    /// ```
    /// 
    /// 
    /// # Arguments
    /// 
    /// * `md` - a middleware to be added.
    /// 
    /// 
    pub fn use_middleware<F>(&mut self, md: F)
    where F: Fn(&HttpRequest) -> () + 'static {
        self.middlewares.push(Box::new(md));
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
    /// app.get("/test".to_string(), |_| {
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
        for md in self.middlewares.iter() {
            md(&req);
        }

            if let Some(method_map) = self.handlers.get(&req.uri.to_string()) {
            match method_map.get(req.method) {
                Some(handler) => handler(req),
                None => HttpResponse::new(StatusCode::NotFound, "Not Found".to_string()),
            }
            } else {
                HttpResponse::new(StatusCode::NotFound,"Not Found".to_string())
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
    ///     app.get("/".to_string(), |_| => {
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
    let mut buffer = [0; 4096];
    let mut request_data = Vec::new();
    let max_request_size = 8192; // Increased for body support
    let mut total_bytes_read = 0;

    // Read headers first
    let headers_end = loop {
        match socket.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by client
                return;
            }
            Ok(n) => {
                request_data.extend_from_slice(&buffer[..n]);
                total_bytes_read += n;

                // Look for the end of headers (\r\n\r\n)
                if let Some(pos) = Self::find_headers_end(&request_data) {
                    break pos + 4; // +4 to include \r\n\r\n
                }

                // Safety: prevent reading too much data
                if total_bytes_read >= max_request_size {
                    eprintln!("Request too large, stopping read");
                    return;
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {:?}", e);
                return;
            }
        }
    };

    // Parse headers
    if let Ok(headers_str) = std::str::from_utf8(&request_data[..headers_end]) {
        match HttpRequest::parse(headers_str) {
            Ok(mut request) => {
                // Parse Content-Length from headers to know how much body to read
                let content_length = Self::get_content_length(headers_str);
                
                // Read the body if there is one
                if content_length > 0 {
                    let mut body = Vec::new();
                    let remaining_bytes = request_data.len() - headers_end;
                    
                    // Add any body data already read
                    if remaining_bytes > 0 {
                        body.extend_from_slice(&request_data[headers_end..]);
                    }

                    // Read the rest of the body
                    let mut bytes_to_read = content_length.saturating_sub(remaining_bytes);
                    while bytes_to_read > 0 {
                        match socket.read(&mut buffer) {
                            Ok(0) => {
                                // Connection closed before reading full body
                                eprintln!("Connection closed before reading full body");
                                break;
                            }
                            Ok(n) => {
                                let to_copy = std::cmp::min(n, bytes_to_read);
                                body.extend_from_slice(&buffer[..to_copy]);
                                bytes_to_read -= to_copy;
                            }
                            Err(e) => {
                                eprintln!("Error reading body from socket: {:?}", e);
                                return;
                            }
                        }
                    }
                    
                    // Convert body to string
                    if let Ok(body_str) = std::str::from_utf8(&body) {
                        request.body = body_str.to_string();
                    } else {
                        eprintln!("Invalid UTF-8 in request body");
                    }
                }

                // Handle the request with body
                let response = self.handle_request(request);
                socket.write_all(response.to_string().as_bytes()).unwrap();
                socket.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Error parsing request: {:?}", e);
            }
        }
    } else {
        eprintln!("Invalid UTF-8 in request headers");
    }
}

// Helper function to find the end of HTTP headers
fn find_headers_end(data: &[u8]) -> Option<usize> {
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

// Helper function to extract Content-Length from headers
fn get_content_length(headers: &str) -> usize {
    for line in headers.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            return line[15..].trim().parse().unwrap_or(0);
        }
    }
    0
}

}


