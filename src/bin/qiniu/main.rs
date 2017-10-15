extern crate futures;
extern crate reqwest;
extern crate tokio_core;

extern crate qiniu;

use std::borrow::Cow;
use std::env;

use futures::Future;

type ReqwestClient = reqwest::unstable::async::Client;


fn main() {
    let ak = env::var("QINIU_RS_TEST_AK").unwrap();
    let sk = env::var("QINIU_RS_TEST_SK").unwrap();

    let mut reactor = tokio_core::reactor::Core::new().unwrap();
    let handle = reactor.handle();
    let https = ReqwestClient::new(&handle);

    let client = qiniu::provider::QiniuClient::new(https, ak, sk);
    let kodo = qiniu::storage::QiniuStorageClient::new(&client);

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
