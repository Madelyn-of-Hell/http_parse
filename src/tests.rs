#[cfg(test)]
mod tests {
    use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::HttpRequest;
    use crate::RequestMethod;

    #[test]
    fn request_from_str() {
        let request = HttpRequest::from_str(&format!("GET /users HTTP1.1\nhost: mads\ndoom: test\n\nbgeoirrrfmeriobody")).unwrap();

        let mut ex_headers = HashMap::new();

        ex_headers.insert("host".to_string(), "mads".to_string());
        ex_headers.insert("doom".to_string(), "test".to_string());

        assert_eq!(
            HttpRequest {
            method: RequestMethod::Get,
            target: PathBuf::from_str("/users").unwrap(),
            headers: ex_headers,
            body: "bgeoirrrfmeriobody".to_string()
        },
            request
        )
    }
}