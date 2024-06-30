#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wasi::http::incoming_handler::Guest;
use bindings::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

impl Guest for Component {
    fn handle(_: IncomingRequest, outparam: ResponseOutparam) {
        let resp = OutgoingResponse::new(Headers::new());

        let body = resp.body().unwrap();
        let output = body.write().unwrap();
        output
            .blocking_write_and_flush(b"Hello, wasi:http/proxy!\n")
            .unwrap();
        drop(output);
        OutgoingBody::finish(body, None).unwrap();

        ResponseOutparam::set(outparam, Ok(resp));
    }
}

bindings::export!(Component with_types_in bindings);
