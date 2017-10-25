use std::borrow::Cow;

#[cfg(feature = "async-api")]
use futures::prelude::*;

use super::super::errors::*;
use super::super::provider;
use super::super::request;
use super::super::reqwest_compat as reqwest;
use super::types;


pub struct QiniuStorageClient<'a> {
    provider: &'a provider::QiniuClient,
}


impl<'a> QiniuStorageClient<'a> {
    pub fn new(provider: &'a provider::QiniuClient) -> QiniuStorageClient<'a> {
        QiniuStorageClient { provider: provider }
    }

    // Flavor-agnostic APIs.

    pub fn upload_token(&self, put_policy: types::PutPolicy) -> String {
        put_policy.into_upload_token(self.provider.signer())
    }
}


impl<'a> QiniuStorageClient<'a> {
    fn req_list_buckets(&self) -> request::QiniuRequest {
        request::QiniuRequest::new(
            reqwest::Method::Get,
            self.provider.hosts().rs().join("buckets").unwrap(),
            None,
        ).unwrap()
    }

    #[cfg(feature = "async-api")]
    pub fn list_buckets(&self) -> impl Future<Item = Vec<String>, Error = Error> {
        let req = self.req_list_buckets();
        // TODO: fix this unwrap
        let x = self.provider.execute(req).unwrap();
        let x = x.and_then(|mut x| x.json()).map_err(|e| e.into());

        x
    }

    #[cfg(feature = "sync-api")]
    pub fn list_buckets(&self) -> Result<Vec<String>> {
        let req = self.req_list_buckets();
        Ok(self.provider.execute(req)?.json()?)
    }
}


/// Type for domains associated with buckets.
///
/// See [the Fusion CDN developer docs][domains] for details.
///
/// [domains]: https://developer.qiniu.com/fusion/kb/1319/test-domain-access-restriction-rules
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BucketDomain {
    /// Test domain automatically provisioned on bucket creation.
    ///
    /// The domain has limited concurrency, speed, and transfer quotas, and as
    /// such is only meant for testing.
    TestDomain(String),
    /// Domain backed by Qiniu Fusion CDN, suitable for production uses.
    FusionDomain(String),
}


impl BucketDomain {
    /// Returns if the domain is meant for testing only, i.e. unsuitable for
    /// production use.
    pub fn is_test_domain(&self) -> bool {
        match self {
            &BucketDomain::TestDomain(_) => true,
            &BucketDomain::FusionDomain(_) => false,
        }
    }
}


impl From<String> for BucketDomain {
    fn from(x: String) -> Self {
        if x.ends_with("bkt.clouddn.com") {
            BucketDomain::TestDomain(x)
        } else {
            BucketDomain::FusionDomain(x)
        }
    }
}


impl Into<String> for BucketDomain {
    fn into(self) -> String {
        match self {
            BucketDomain::TestDomain(x) => x,
            BucketDomain::FusionDomain(x) => x,
        }
    }
}


impl ::std::ops::Deref for BucketDomain {
    type Target = str;

    fn deref(&self) -> &str {
        match self {
            &BucketDomain::TestDomain(ref x) => x,
            &BucketDomain::FusionDomain(ref x) => x,
        }
    }
}


impl<'a> QiniuStorageClient<'a> {
    fn req_bucket_domains<'b: 'a>(&'a self, bucket: Cow<'b, str>) -> request::QiniuRequest {
        let url = {
            let mut tmp = self.provider.hosts().api().join("v6/domain/list").unwrap();
            {
                let mut qs = tmp.query_pairs_mut();
                qs.append_pair("tbl", bucket.as_ref());
            }
            tmp
        };

        request::QiniuRequest::new(reqwest::Method::Get, url, None).unwrap()
    }

    #[cfg(feature = "async-api")]
    pub fn bucket_domains<'b: 'a>(
        &'a self,
        bucket: Cow<'b, str>,
    ) -> impl Future<Item = Vec<BucketDomain>, Error = Error> {
        let req = self.req_bucket_domains(bucket);
        // TODO
        let x = self.provider.execute(req).unwrap();
        let x = x.and_then(|mut x| {
            x.json().map(|l: Vec<String>| {
                l.into_iter().map(|d| d.into()).collect()
            })
        }).map_err(|e| e.into());

        x
    }

    #[cfg(feature = "sync-api")]
    pub fn bucket_domains<'b: 'a>(&'a self, bucket: Cow<'b, str>) -> Result<Vec<BucketDomain>> {
        let req = self.req_bucket_domains(bucket);
        let resp: Vec<String> = self.provider.execute(req)?.json()?;
        Ok(resp.into_iter().map(|d| d.into()).collect())
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
    pub end_user: Option<String>,
}


impl<'a> QiniuStorageClient<'a> {
    fn req_bucket_list<'b: 'a>(
        &'a self,
        bucket: Cow<'b, str>,
        limit: Option<usize>,
        prefix: Option<&'b str>,
        delimiter: Option<&'b str>,
        marker: Option<&'b str>,
    ) -> request::QiniuRequest {
        let url = {
            let mut tmp = self.provider.hosts().rsf().join("list").unwrap();
            {
                let mut qs = tmp.query_pairs_mut();

                qs.append_pair("bucket", bucket.as_ref());
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

        request::QiniuRequest::new(reqwest::Method::Post, url, None).unwrap()
    }

    #[cfg(feature = "async-api")]
    pub fn bucket_list<'b: 'a>(
        &'a self,
        bucket: Cow<'b, str>,
        limit: Option<usize>,
        prefix: Option<&'b str>,
        delimiter: Option<&'b str>,
        marker: Option<&'b str>,
    ) -> impl Future<Item = ListResponse, Error = Error> {
        let req = self.req_bucket_list(bucket, limit, prefix, delimiter, marker);
        // TODO: fix this unwrap
        let x = self.provider.execute(req).unwrap();
        let x = x.and_then(|mut x| x.json()).map_err(|e| e.into());

        x
    }

    #[cfg(feature = "sync-api")]
    pub fn bucket_list<'b: 'a>(
        &'a self,
        bucket: Cow<'b, str>,
        limit: Option<usize>,
        prefix: Option<&'b str>,
        delimiter: Option<&'b str>,
        marker: Option<&'b str>,
    ) -> Result<ListResponse> {
        let req = self.req_bucket_list(bucket, limit, prefix, delimiter, marker);
        Ok(self.provider.execute(req)?.json()?)
    }
}
