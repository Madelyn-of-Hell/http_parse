mod tests;
use std::{collections::HashMap, fmt::Display, path::PathBuf};
use regex::Regex;
use anyhow::{Ok, Result, anyhow};
use std::result::Result::Ok as StdOk;

/// An HTTP Request, typically parsed from a serialised request.
#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
    method: RequestMethod,
    target: PathBuf,
    query: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Vec<u8>
}
impl HttpRequest {
    /// Returns a new HttpRequest as long as the bytes comprise a valid HTTP request.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        Self::from_str(&String::from_utf8_lossy(slice))
    }
    /// Returns a new HttpRequest as long as the string comprise a valid HTTP request.
    pub fn from_str(string: &str) -> Result<Self> {
        let top_level_splitter = Regex::new(r"(?<method>.+?) (?<target>.+?) .+\s(?<headers>(?:.+\s)+)\s+(?<body>[\s\S]+)?")?;
        let header_splitter = Regex::new(r"(?<key>.+): (?<value>\S+)\s?")?;
        let query_splitter = Regex::new(r"(?<key>[^&]+)=(?<value>[^&]+)&?")?;

        let top_level_captures = top_level_splitter.captures(string).ok_or(anyhow!("Failed to parse the Request"))?;

        let method_raw  = top_level_captures.name("method") .map(|capture| capture.as_str()).ok_or(anyhow!("No method found"))?;
        let target_raw  = top_level_captures.name("target") .map(|capture| capture.as_str()).ok_or(anyhow!("No target found"))?;
        let headers_raw = top_level_captures.name("headers").map(|capture| capture.as_str()).ok_or(anyhow!("No headers found"))?;
        let body     = top_level_captures.name("body")   .map(|capture| capture.as_str()).unwrap_or("").to_string().into_bytes();
        let (target_raw, query_raw) = target_raw.split_once("?").unwrap_or((target_raw,""));

        let method = RequestMethod::try_from(method_raw).ok_or(anyhow!("Invalid method: {method_raw}"))?;
        let target = PathBuf::from(target_raw);
        
        let query_captures = query_splitter.captures_iter(query_raw);
        let query = query_captures.filter_map( |entry | {
                let Some(header) = entry.name("key").map(|capture| capture.as_str()) else {return None};
                let Some(value) = entry.name("value").map(|capture| capture.as_str()) else {return None};
                Some((header.to_string(), value.to_string()))
            }
        ).collect::<HashMap<String, String>>();
        
        let header_captures = header_splitter.captures_iter(headers_raw);
        let headers = header_captures.filter_map( |entry | {
                let Some(header) = entry.name("key").map(|capture| capture.as_str()) else {return None};
                let Some(value) = entry.name("value").map(|capture| capture.as_str()) else {return None};
                Some((header.to_string(), value.to_string()))
            }
        ).collect::<HashMap<String, String>>();
        
        Ok(HttpRequest {
            method,
            target,
            query,
            headers,
            body
        })
    }

    pub fn method(&self) -> RequestMethod {
        self.method.clone()
    }
    pub fn target(&self) -> PathBuf {
        self.target.clone()
    }
    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}
impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query_str = self.query.iter().map(|(key, value)| format!("{key}={value}")).collect::<Vec<String>>().join("&");
        write!(f, "{} {}?{} HTTP/1.1\r\n", self.method, self.target.display(), query_str)?;
        for (key, value) in &self.headers {
            write!(f, "{key}: {value}\r\n")?;
        };

        write!(f, "\r\n")?;
        
        if let StdOk(body_string) = String::from_utf8(self.body.clone()) {
            write!(f, "{}", body_string)
        }
        else {
            write!(f, "{}", self.body.iter().map(|byte| *byte as char).collect::<String>())
        }
    }
}

/// An HTTP Response, Typically built with the builder methods and then serialised.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponse {
    pub status_code: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}
impl HttpResponse {
    /// Construct a new, empty response.
    pub fn new() -> Self {
        Self {
            status_code: HttpStatus::Ok,
            headers: HashMap::new(),
            body: vec![]
        }
    }
    /// Return the response with the given status.
    pub fn with_status(self, status: HttpStatus) -> Self {
        let mut new = self.clone();
        new.status_code = status;

        new
    }
    /// Return the response with the given header.
    pub fn with_header(self, key: String, value: String) -> Self {
        let mut new = self.clone();
        new.headers.insert(key, value);

        new
    } 
    /// Return the response with the given body.
    pub fn with_body_bytes(self, body: &[u8]) -> Self {
        let mut new = self.clone();
        new.body = body.to_vec();

        new
    }
    /// Return the response with the given status.
    pub fn with_body_string(self, body: String) -> Self {
        let mut new = self.clone();
        new.body = body.into_bytes();

        new
    }
}
impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"HTTP/1.1 {}\r\n", self.status_code as u16)?;
        let mut headers = self.headers.clone();
        if let Some(server) = headers.remove("Server") {
            write!(f, "Server: {server}\r\n")?;
        } else {
            write!(f, "Server: Rust Custom\r\n")?;
        }

        if !self.body.is_empty() {
            if let Some(content_length) = headers.remove("Content-Length") {
                write!(f, "Content-Length: {content_length}\r\n")?;
            } else {
                write!(f, "Content-Length: {}\r\n", self.body.len())?;
            }
        }

        for (key, value) in &self.headers {
            write!(f, "{key}: {value}\r\n")?;
        }

        write!(f, "\r\n")?;
        
        if let StdOk(body_string) = String::from_utf8(self.body.clone()) {
            write!(f, "{}", body_string)
        }
        else {
            write!(f, "{}", self.body.iter().map(|byte| *byte as char).collect::<String>())
        }    
    }
}

/// Methods for an HTTP Request

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RequestMethod {
    Get,
    Query,
    Head,
    Post,
    Put,
    Delete
}
impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Query => "QUERY",
                Self::Head => "HEAD",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Delete => "DELETE",
            }
        )
    }
}
impl RequestMethod {
    /// Return the matching enum variant, if applicable.
    pub fn try_from(from: &str) -> Option<RequestMethod> {
        return match from.to_lowercase().as_str() {
            "get" => Some(Self::Get),
            "query" => Some(Self::Query),
            "head" => Some(Self::Head),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "delete" => Some(Self::Delete),
            _ => None
        }
    }
}

/// All HTTP Status Codes.
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    Checkpoint = 103,
    RequestURITooLong = 122,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImAteapot = 418,
    MisdirectedRequest = 421,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NetworkAuthenticationRequired = 511
}
