use crate::lib::{
    req_res_structs::{Request, Response},
    server_errors::ServerError,
};

pub fn parse_request(req_body: String) -> Result<Request, ServerError> {
    Ok(Request::default())
}

pub fn deser_response(response: Response) -> String {
    String::default()
}
