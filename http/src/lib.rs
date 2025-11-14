pub mod server;
pub mod app;
pub mod jsonable;




#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::server::{HttpRequest, Respondable, StatusCode};



    #[test]
    fn test_app() {
        let mut app = crate::app::App::new();

        app.add_handler("/".to_string(),|_| {
            "Hello world".into_response()
        });

        let req = HttpRequest {
            method:"GET",
            uri: "/",
            version: "HTTP/1.1",
            headers:HashMap::new(),
            body:String::new(),
            content_length:0,
        };

        let response = app.handle_request(req);

        assert_eq!(response.body,"Hello world");
        assert_eq!(response.status,StatusCode::Ok);

        
    }
}