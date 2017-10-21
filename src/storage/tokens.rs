//! [Upload tokens][upload-token] and [download tokens][download-token].
//!
//! [upload-token]: https://developer.qiniu.com/kodo/manual/1208/upload-token
//! [download-token]: https://developer.qiniu.com/kodo/manual/1202/download-token

use base64;
use serde_json;

use super::super::sign;
use super::types::PutPolicy;


impl PutPolicy {
    pub(crate) fn into_upload_token(self, signer: &sign::QiniuSigner) -> String {
        let json = serde_json::to_string(&self).unwrap();
        let encoded = base64::encode_config(json.as_bytes(), base64::URL_SAFE);

        let mut sign = signer.sign_blob(encoded.as_bytes());
        sign.push(':');
        sign.push_str(&encoded);

        sign
    }
}


#[cfg(test)]
mod tests {
    use super::super::super::sign;
    use super::super::types;

    #[test]
    fn test_put_policy() {
        let signer = sign::QiniuSigner::new("MY_ACCESS_KEY", "MY_SECRET_KEY");
        let scope = types::PutScope::BucketKey("my-bucket".to_owned(), "sunflower.jpg".to_owned());
        let b = r#"{"name":$(fname),"size":$(fsize),"w":$(imageInfo.width),"h":$(imageInfo.height),"hash":$(etag)}"#;
        let pp = types::PutPolicyBuilder::new(scope, 1451491200)
            .return_body(b.to_owned())
            .build();
        let result = pp.into_upload_token(&signer);

        assert_eq!(
            result,
            "MY_ACCESS_KEY:wQ4ofysef1R7IKnrziqtomqyDvI=:eyJzY29wZSI6Im15LWJ1Y2tldDpzdW5mbG93ZXIuanBnIiwiZGVhZGxpbmUiOjE0NTE0OTEyMDAsInJldHVybkJvZHkiOiJ7XCJuYW1lXCI6JChmbmFtZSksXCJzaXplXCI6JChmc2l6ZSksXCJ3XCI6JChpbWFnZUluZm8ud2lkdGgpLFwiaFwiOiQoaW1hZ2VJbmZvLmhlaWdodCksXCJoYXNoXCI6JChldGFnKX0ifQ=="
        );
    }
}
