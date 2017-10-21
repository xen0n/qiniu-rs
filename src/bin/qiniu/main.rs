#[cfg(feature = "async-api")]
extern crate futures;
extern crate reqwest;
extern crate tokio_core;

extern crate qiniu;

use std::borrow::Cow;
use std::env;

#[cfg(feature = "async-api")]
use futures::Future;


fn main() {
    let ak = env::var("QINIU_RS_TEST_AK").unwrap();
    let sk = env::var("QINIU_RS_TEST_SK").unwrap();

    #[cfg(feature = "async-api")]
    let mut reactor = tokio_core::reactor::Core::new().unwrap();
    #[cfg(feature = "async-api")]
    let handle = reactor.handle();
    #[cfg(feature = "async-api")]
    let client = qiniu::provider::QiniuClient::new(&handle, ak, sk);

    #[cfg(feature = "sync-api")]
    let client = qiniu::provider::QiniuClient::new(ak, sk);

    let kodo = qiniu::storage::QiniuStorageClient::new(&client);

    #[cfg(feature = "async-api")]
    {
        let req = kodo.list_buckets();
        let req = req.and_then(|r| {
            println!("list buckets response = {:?}", r);

            let test_bucket_name = r[0].to_owned();
            kodo.bucket_list(Cow::Owned(test_bucket_name.clone()), None, None, None, None)
                .map(move |l| (test_bucket_name, l))
        });
        let req = req.and_then(|(bkt, list)| {
            println!("list inside bucket = {:?}", list);

            kodo.bucket_domains(Cow::Owned(bkt.clone())).map(
                |l| (bkt, l),
            )
        });
        let req = req.and_then(|(bkt, domains)| {
            println!("domains of bucket {}: {:?}", bkt, domains);

            Ok(())
        });

        reactor.run(req).unwrap();
    }

    #[cfg(feature = "sync-api")]
    {
        let buckets = kodo.list_buckets().unwrap();
        println!("list buckets response = {:?}", buckets);
        let list = kodo.bucket_list(Cow::Borrowed(&buckets[0]), None, None, None, None)
            .unwrap();
        println!("list inside bucket = {:?}", list);
        let domains = kodo.bucket_domains(Cow::Borrowed(&buckets[0])).unwrap();
        println!("domains of bucket {}: {:?}", &buckets[0], domains);
    }
}
