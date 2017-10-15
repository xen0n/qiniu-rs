use futures::prelude::*;

use super::errors::*;
use super::provider;
use super::request;
use super::reqwest_compat as reqwest;


pub struct QiniuStorageClient<'a> {
    provider: &'a provider::QiniuClient,
}


impl<'a> QiniuStorageClient<'a> {
    pub fn new(provider: &'a provider::QiniuClient) -> QiniuStorageClient<'a> {
        QiniuStorageClient {
            provider: provider,
        }
    }

    pub fn list_buckets(&self) -> impl Future<Item=Vec<String>, Error=Error> {
        let req = request::QiniuRequest::new(
            reqwest::Method::Get,
            self.provider.hosts().rs().join("buckets").unwrap(),
            None,
            ).unwrap();

        // TODO: fix this unwrap
        let x = self.provider.execute(req).unwrap();
        let x = x.and_then(|mut x| x.json()).map_err(|e| e.into());

        x
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse {
    pub marker: Option<String>,
    pub common_prefixes: Option<Vec<String>>,
    pub items: Vec<ListBucketEntry>,
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBucketEntry {
    pub key: String,
    pub put_time: u64,
    pub fsize: u64,
    pub hash: String,
    pub mime_type: String,
    #[serde(rename = "type")]
    pub type_: usize,
    pub end_user: String,
}


impl<'a> QiniuStorageClient<'a> {
    pub fn bucket_list<'b: 'a>(&'a self,
                               bucket: &'b str,
                               limit: Option<usize>,
                               prefix: Option<&'b str>,
                               delimiter: Option<&'b str>,
                               marker: Option<&'b str>,
                               ) -> impl Future<Item=ListResponse, Error=Error>
    {
        let url = {
            let mut tmp = self.provider.hosts().rsf().join("list").unwrap();
            {
                let mut qs = tmp.query_pairs_mut();

                qs.append_pair("bucket", bucket);
                if let Some(limit) = limit {
                    qs.append_pair("limit", &format!("{}", limit));
                }
                if let Some(prefix) = prefix {
                    qs.append_pair("prefix", prefix);
                }
                if let Some(delimiter) = delimiter {
                    qs.append_pair("delimiter", delimiter);
                }
                if let Some(marker) = marker {
                    qs.append_pair("marker", marker);
                }
            }
            tmp
        };
        let req = request::QiniuRequest::new(
            reqwest::Method::Post,
            url,
            None,
            ).unwrap();

        // TODO: fix this unwrap
        let x = self.provider.execute(req).unwrap();
        let x = x.and_then(|mut x| x.json()).map_err(|e| e.into());

        x
    }
}
