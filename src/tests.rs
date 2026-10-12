#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::HttpRequest;
    use crate::HttpResponse;
    use crate::HttpStatus::InternalServerError;
    use crate::RequestMethod;

    #[test]
    fn parse_request_from_str() {
        let request = HttpRequest::from_str("GET /users HTTP/1.1\nhost: mads\ndoom: test\n\nbgeoirrrfmeriobody").unwrap();

        let mut ex_headers = HashMap::new();

        ex_headers.insert("host".to_string(), "mads".to_string());
        ex_headers.insert("doom".to_string(), "test".to_string());

        assert_eq!(
            HttpRequest {
            method: RequestMethod::Get,
            target: PathBuf::from_str("/users").unwrap(),
            headers: ex_headers,
            body: "bgeoirrrfmeriobody".to_string().into_bytes()
        },
            request
        )
    }

    #[test]
    fn serialise_request() {
        let mut ex_headers = HashMap::new();

        ex_headers.insert("host".to_string(), "mads".to_string());
        ex_headers.insert("doom".to_string(), "test".to_string());

        let request = HttpRequest {
            method: RequestMethod::Get,
            target: PathBuf::from_str("/users").unwrap(),
            headers: ex_headers,
            body: "bgeoirrrfmeriobody".to_string().into_bytes()
        };
        
        let request_string = request.to_string();
        println!("{request_string}");
        assert!(request_string.starts_with("GET /users HTTP/1.1\r\n"));
        assert!(request_string.ends_with("\r\n\r\nbgeoirrrfmeriobody"));
        assert!(request_string.contains("\r\nhost: mads"));
        assert!(request_string.contains("\r\ndoom: test"));
    }

   
    #[test]
    fn response_with_constructors() {
        let response = HttpResponse::new().with_status(InternalServerError).with_header("Hii".to_string(), "There".to_string()).with_body_string("MY SECRET INFORMATION NOBODY CAN SEE".to_string());

        let mut ex_headers = HashMap::new();
        ex_headers.insert("Hii".to_string(), "There".to_string());
        assert_eq!(
            HttpResponse {
                status_code: InternalServerError,
                headers: ex_headers,
                body: "MY SECRET INFORMATION NOBODY CAN SEE".to_string().into_bytes()
            },
            response
        )
    }

    #[test]
    fn serialise_response() {
        let response = HttpResponse::new().with_status(InternalServerError).with_header("Hii".to_string(), "There".to_string()).with_body_string("MY SECRET INFORMATION NOBODY CAN SEE".to_string());

        assert_eq!(
            "HTTP/1.1 500\r\nServer: Rust Custom\r\nContent-Length: 36\r\nHii: There\r\n\r\nMY SECRET INFORMATION NOBODY CAN SEE",
            response.to_string()
        )
    }
}