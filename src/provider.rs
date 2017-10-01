use hyper;
use url;

use super::sign;
use super::request;


pub struct QiniuClient<T> {
    signer: sign::QiniuSigner,
    client: hyper::Client<T, hyper::Body>,

    hosts: QiniuHosts,
}

pub struct QiniuHosts {
    rs: url::Url,
}


impl Default for QiniuHosts {
    fn default() -> QiniuHosts {
        QiniuHosts {
            rs: "https://rs.qiniu.com".parse().unwrap(),
        }
    }
}


impl QiniuHosts {
    pub fn rs(&self) -> &url::Url {
        &self.rs
    }
}


impl<T: hyper::client::Connect> QiniuClient<T> {
    pub fn new<AK, SK>(client: hyper::Client<T, hyper::Body>, ak: AK, sk: SK) -> QiniuClient<T>
        where AK: AsRef<str>, SK: AsRef<str>
    {
        let signer = sign::QiniuSigner::new(ak, sk);

        QiniuClient {
            signer: signer,
            client: client,
            hosts: QiniuHosts::default(),
        }
    }

    pub fn execute(&self, req: request::QiniuRequest) -> hyper::client::FutureResponse {
        let hyper_req = req.into_hyper(&self.signer);

        self.client.request(hyper_req)
    }
}


impl<T> QiniuClient<T> {
    pub fn hosts(&self) -> &QiniuHosts {
        &self.hosts
    }
}
