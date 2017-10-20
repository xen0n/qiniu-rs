use url;

use super::errors::*;
use super::sign;
use super::request;
use super::reqwest_compat as reqwest;


pub struct QiniuClient {
    signer: sign::QiniuSigner,
    client: reqwest::Client,

    hosts: QiniuHosts,
}

pub struct QiniuHosts {
    rs: url::Url,
    rsf: url::Url,
    api: url::Url,
}


impl Default for QiniuHosts {
    fn default() -> QiniuHosts {
        QiniuHosts {
            rs: "https://rs.qiniu.com".parse().unwrap(),
            rsf: "https://rsf.qiniu.com".parse().unwrap(),
            api: "https://api.qiniu.com".parse().unwrap(),
        }
    }
}


impl QiniuHosts {
    pub fn rs(&self) -> &url::Url {
        &self.rs
    }

    pub fn rsf(&self) -> &url::Url {
        &self.rsf
    }

    pub fn api(&self) -> &url::Url {
        &self.api
    }
}


impl QiniuClient {
    pub fn new<AK, SK>(client: reqwest::Client, ak: AK, sk: SK) -> QiniuClient
    where
        AK: AsRef<str>,
        SK: AsRef<str>,
    {
        let signer = sign::QiniuSigner::new(ak, sk);

        QiniuClient {
            signer: signer,
            client: client,
            hosts: QiniuHosts::default(),
        }
    }

    pub(crate) fn signer(&self) -> &sign::QiniuSigner {
        &self.signer
    }

    pub(crate) fn reqwest_client(&self) -> &reqwest::Client {
        &self.client
    }

    #[cfg(feature = "async-api")]
    pub fn execute(
        &self,
        req: request::QiniuRequest,
    ) -> Result<impl ::futures::Future<Item = reqwest::Response, Error = reqwest::Error>> {
        let ll_req = req.into_lowlevel(self)?;

        Ok(self.client.execute(ll_req))
    }

    #[cfg(feature = "sync-api")]
    pub fn execute(&self, req: request::QiniuRequest) -> Result<reqwest::Response> {
        let ll_req = req.into_lowlevel(self)?;

        Ok(self.client.execute(ll_req)?)
    }
}


impl QiniuClient {
    pub fn hosts(&self) -> &QiniuHosts {
        &self.hosts
    }
}
