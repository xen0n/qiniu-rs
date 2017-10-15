extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

extern crate qiniu;

use std::env;

use futures::Future;


fn main() {
    let ak = env::var("QINIU_RS_TEST_AK").unwrap();
    let sk = env::var("QINIU_RS_TEST_SK").unwrap();

    let mut reactor = tokio_core::reactor::Core::new().unwrap();
    let handle = reactor.handle();
    let https = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(2, &handle).unwrap())
        .build(&handle);

    let client = qiniu::provider::QiniuClient::new(https, ak, sk);
    let kodo = qiniu::storage::QiniuStorageClient::new(&client);

    let req = kodo.list_buckets();
    let req = req.and_then(|r| {
        println!("list buckets response = {:?}", r);

        let test_bucket_name = &r[0];
        kodo.bucket_list(test_bucket_name, None, None, None, None)
    });
    let req = req.and_then(|list| {
        println!("list inside bucket = {:?}", list);

        Ok(())
    });

    reactor.run(req).unwrap();
}
