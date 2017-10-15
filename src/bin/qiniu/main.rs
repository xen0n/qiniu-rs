#[cfg(feature = "async-api")]
extern crate futures;
extern crate reqwest;
extern crate tokio_core;

extern crate qiniu;

use std::borrow::Cow;
use std::env;

#[cfg(feature = "async-api")]
use futures::Future;

#[cfg(feature = "async-api")]
type ReqwestClient = reqwest::unstable::async::Client;
#[cfg(feature = "sync-api")]
type ReqwestClient = reqwest::Client;


fn main() {
    let ak = env::var("QINIU_RS_TEST_AK").unwrap();
    let sk = env::var("QINIU_RS_TEST_SK").unwrap();

    #[cfg(feature = "async-api")]
    let mut reactor = tokio_core::reactor::Core::new().unwrap();
    #[cfg(feature = "async-api")]
    let handle = reactor.handle();
    #[cfg(feature = "async-api")]
    let https = ReqwestClient::new(&handle);

    #[cfg(feature = "sync-api")]
    let https = ReqwestClient::new();

    let client = qiniu::provider::QiniuClient::new(https, ak, sk);
    let kodo = qiniu::storage::QiniuStorageClient::new(&client);

    #[cfg(feature = "async-api")]
    {
        let req = kodo.list_buckets();
        let req = req.and_then(|r| {
            println!("list buckets response = {:?}", r);

            let test_bucket_name = &r[0];
            kodo.bucket_list(Cow::Owned(test_bucket_name.to_owned()), None, None, None, None)
        });
        let req = req.and_then(|list| {
            println!("list inside bucket = {:?}", list);

            Ok(())
        });

        reactor.run(req).unwrap();
    }

    #[cfg(feature = "sync-api")]
    {
        let buckets = kodo.list_buckets().unwrap();
        println!("list buckets response = {:?}", buckets);
        let list = kodo.bucket_list(Cow::Borrowed(&buckets[0]), None, None, None, None).unwrap();
        println!("list inside bucket = {:?}", list);
    }
}
