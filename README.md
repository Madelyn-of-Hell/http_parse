# HTTP_PARSE
A small utility crate for parsing and building HTTP messages.

> [!NOTE]
> This tool is being built at my own convenience, as I need features in other projects. If you'd like a feature that's not currently implemented, make an issue to request it. Better yet, do it yourself and then pull make a pull request.

## Requests
Requests can be parsed from a string or byte slice as follows:
```rust
let request = HttpRequest::from_str("GET /users HTTP/1.1\nhost: mads\ndoom: test\n\nbgeoirrrfmeriobody").unwrap();
```
They can also be serialised:
```rust
println!("{}", request.to_string());
```
## Responses
Responses can be built using the builder methods:
```rust
let response = HttpResponse::new()
    .with_status(InternalServerError)
    .with_header("Hii".to_string(), "There".to_string())
    .with_body_string("MY SECRET INFORMATION NOBODY CAN SEE".to_string());
```
They can then be serialised the same way as requests:
```rust
println!("{}", response.to_string());
```

