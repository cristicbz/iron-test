use iron::prelude::*;
use iron::response::{ResponseBody};

use std::string::FromUtf8Error;

/// Extracts a utf8 response body to a String.
pub fn extract_body_to_string(response: Response) -> Result<String, FromUtf8Error> {
    let result = extract_body_to_bytes(response);
    String::from_utf8(result)
}

/// Extracts a response body to a Vector of bytes.
pub fn extract_body_to_bytes(response: Response) -> Vec<u8> {
    let mut result = Vec::new();

    {
        let mut response_body = ResponseBody::new(&mut result);
        match response.body {
            Some(mut body) => body.write_body(&mut response_body).ok(),
            None => None,
        };
    }

    result
}

#[cfg(test)]
mod test {
    use iron::headers::Headers;
    use iron::prelude::*;
    use iron::{Handler, status};

    use request;

    use super::*;

    struct HelloWorldHandler;

    impl Handler for HelloWorldHandler {
        fn handle(&self, _: &mut Request) -> IronResult<Response> {
            Ok(Response::with((status::Ok, "Hello, world!")))
        }
    }

    #[test]
    fn test_extract_body_to_string() {
        let response = request::get("http://localhost:3000",
                           Headers::new(),
                           HelloWorldHandler);
        let result = extract_body_to_string(response.unwrap());

        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[test]
    fn test_extract_body_to_bytes() {
        let response = request::get("http://localhost:3000",
                           Headers::new(),
                           HelloWorldHandler);
        let result = extract_body_to_bytes(response.unwrap());

        assert_eq!(result, b"Hello, world!");
    }
}
