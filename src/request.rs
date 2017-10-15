use bytes;
use url;

use super::provider;
use super::reqwest_compat as reqwest;

use super::errors::*;


pub struct QiniuRequest {
    method: reqwest::Method,
    uri: url::Url,
    body: Option<bytes::Bytes>,
}


impl QiniuRequest {
    pub fn new<S: AsRef<str>>(method: reqwest::Method, uri: S, body: Option<bytes::Bytes>) -> Result<QiniuRequest> {
        Ok(QiniuRequest {
            method: method,
            uri: url::Url::parse(uri.as_ref())?,
            body: body,
        })
    }

    pub (crate) fn into_lowlevel(self, client: &provider::QiniuClient) -> Result<reqwest::Request> {
        let mut builder = client.reqwest_client().request(self.method, self.uri.as_ref());

        // sign request
        let auth_hdr = {
            let signer = client.signer();
            let mut tmp = String::from("QBox ");
            // this clone is lightweight (maybe? due to the Arc inside)
            let auth = signer.sign(&self.uri, self.body.clone().as_ref().map(|buf| &buf[..]));
            tmp.push_str(&auth);
            tmp
        };

        let builder = builder.header(reqwest::header::Authorization(auth_hdr));

        let builder = if let Some(body) = self.body {
            builder.body(body)
        } else {
            builder
        };

        Ok(builder.build()?)
    }
}
