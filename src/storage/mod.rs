mod buckets;


use futures::prelude::*;
use hyper;
use serde_json;

use super::errors::*;
use super::provider;


pub struct QiniuStorageClient<'a, T: 'a> {
    provider: &'a provider::QiniuClient<T>,
}


impl<'a, T: hyper::client::Connect> QiniuStorageClient<'a, T> {
    pub fn new(provider: &'a provider::QiniuClient<T>) -> QiniuStorageClient<'a, T> {
        QiniuStorageClient {
            provider: provider,
        }
    }

    pub fn list_buckets(&self) -> Box<Future<Item=Vec<String>, Error=Error>> {
        let req = buckets::list_buckets(self.provider);

        let x = self.provider.execute(req);
        let x = x.map_err(|e| e.into());
        let x = x.and_then(|res| res.body().concat2().map_err(|e| e.into()));
        let x = x.and_then(|body| serde_json::from_slice(&body).map_err(|e| e.into()));

        Box::new(x)
    }
}
