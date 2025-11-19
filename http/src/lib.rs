


/// Server module.
/// 
/// Contains server utilities.
mod server;


/// App module.
/// 
/// Contains the basic module builder.
mod app;

/// Jsonable module.
/// 
/// 
/// Contains `Jsonable` trait required for serialization and deserialization.
pub mod jsonable;



pub use jsonable::{Jsonable,Parser};


pub use json::Jsonable;
pub use app::{App,MiddlewareResult,Middleware,Handler};
pub use server::{Respondable,HttpRequest,HttpResponse,StatusCode};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::server::{HttpRequest, Respondable, StatusCode};



    #[test]
    fn test_app() {
        let mut app = crate::app::App::new();

        app.get("/".to_string(),|_| {
            "Hello world".into_response()
        });

        let req = HttpRequest {
            method:String::from("GET"),
            uri: String::from("/"),
            version: String::from("HTTP/1.1"),
            headers:HashMap::new(),
            body:String::new(),
            content_length:0,
            path_params:HashMap::new()
        };

        let response = app.handle_request(req);

        assert_eq!(response.body,"Hello world");
        assert_eq!(response.status,StatusCode::Ok);

        
    }
}