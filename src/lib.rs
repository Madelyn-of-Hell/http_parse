mod tests;
use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};
use log::{debug, error};
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
    method: RequestMethod,
    target: PathBuf,
    headers: HashMap<String, String>,
    body: String
}
impl HttpRequest {
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        Self::from_str(&String::from_utf8_lossy(slice))
    }
    pub fn from_str(string: &str) -> Option<Self> {
        let Ok(top_level_splitter) = Regex::new(r"(?<method>.+?) (?<target>.+?) .+\s(?<headers>(?:.+\s)+)\s(?<body>[\s\S]+)?") else {
            error!("Top Level  failed to compile");
            return None
        };
        let Ok(header_splitter) = Regex::new(r"(?<key>.+): (?<value>.+)\s?") else {
            error!("Top Level  failed to compile");
            return None
        };
        let Some(top_level_captures) = top_level_splitter.captures(string) else {debug!("Couldn't capture anything!"); return None};

        let Some(method_raw)  = top_level_captures.name("method") .map(|capture| capture.as_str()) else {debug!("No method found");  return None};
        let Some(target_raw)  = top_level_captures.name("target") .map(|capture| capture.as_str()) else {debug!("No target found");  return None};
        let Some(headers_raw) = top_level_captures.name("headers").map(|capture| capture.as_str()) else {debug!("No headers found"); return None};
        let body              = top_level_captures.name("body").map(|capture| capture.as_str()).unwrap_or("").to_string();

        let Some(method) = RequestMethod::try_from(method_raw) else {debug!("Invalid method: {method_raw}"); return None};
        let Ok(target) = PathBuf::from_str(target_raw) else {debug!("Target couldn't be formatted to a path. Value: {target_raw}"); return None};
        
        let header_captures = header_splitter.captures_iter(headers_raw);
        let headers = header_captures.filter_map( |entry | {
                let Some(header) = dbg!(entry.name("key").map(|capture| capture.as_str())) else {error!("No key found for the current entry"); return None};
                let Some(value) = dbg!(entry.name("value").map(|capture| capture.as_str())) else {error!("No value found for the current entry"); return None};
                Some((header.to_string(), value.to_string()))
            }
        ).collect::<HashMap<String, String>>();
        
        Some(HttpRequest {
            method,
            target,
            headers,
            body
        })
    }
}
impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} HTTP/1.1", self.method, self.target.display())?;
        for (key, value) in &self.headers {
            writeln!(f, "{key}: {value}")?;
        };
        write!(f, "\n{}", self.body)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpResponse {
    status_code: HttpStatus,
    headers: HashMap<String, String>,
    body: String
}

#[derive(Debug, PartialEq, Eq)]
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

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    Checkpoint = 103,
    RequestURITooLong = 122,
    OK = 200,
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
}
