use bytes;
use hyper;
use url;

use super::sign;

use super::errors::*;


pub struct QiniuRequest {
    method: hyper::Method,
    uri: url::Url,
    body: Option<bytes::Bytes>,
}


impl QiniuRequest {
    pub fn new<S: AsRef<str>>(method: hyper::Method, uri: S, body: Option<bytes::Bytes>) -> Result<QiniuRequest> {
        Ok(QiniuRequest {
            method: method,
            uri: url::Url::parse(uri.as_ref())?,
            body: body,
        })
    }

    pub (crate) fn into_hyper(self, signer: &sign::QiniuSigner) -> hyper::Request<hyper::Body> {
        let mut req = hyper::Request::new(self.method, self.uri.as_ref().parse().unwrap());

        // sign request
        {
            let auth_hdr = {
                let mut tmp = String::from("QBox ");
                // this clone is lightweight (maybe? due to the Arc inside)
                let auth = signer.sign(&self.uri, self.body.clone().as_ref().map(|buf| &buf[..]));
                tmp.push_str(&auth);
                tmp
            };

            let headers = req.headers_mut();
            headers.set(hyper::header::Authorization(auth_hdr));
        }

        if let Some(body) = self.body {
            req.set_body(body);
        }

        req
    }
}
