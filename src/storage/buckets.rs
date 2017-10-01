use hyper;

use super::super::provider;
use super::super::request;


pub fn list_buckets<T>(provider: &provider::QiniuClient<T>) -> request::QiniuRequest {
    request::QiniuRequest::new(
        hyper::Method::Get,
        provider.hosts().rs().join("buckets").unwrap(),
        None,
        ).unwrap()
}
