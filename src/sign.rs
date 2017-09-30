use base64;
use ring;
use url;


pub struct QiniuSigner<'a> {
    ak: &'a str,
    sk: ring::hmac::SigningKey,
}


impl<'a> QiniuSigner<'a> {
    pub fn new(ak: &'a str, sk: &'a str) -> QiniuSigner<'a> {
        let key = ring::hmac::SigningKey::new(&ring::digest::SHA1, sk.as_bytes());

        QiniuSigner {
            ak: ak,
            sk: key,
        }
    }

    pub fn sign(&'a self, url: &url::Url, body: Option<&[u8]>) -> String {
        let mut ctx = ring::hmac::SigningContext::with_key(&self.sk);
        ctx.update(url.path().as_bytes());
        if let Some(qs) = url.query() {
            ctx.update(b"?");
            ctx.update(qs.as_bytes());
        }
        ctx.update(b"\n");
        if let Some(body) = body {
            ctx.update(body);
        }

        let digest = ctx.sign();
        let digest = digest.as_ref();

        {
            let mut tmp = String::from(self.ak);
            tmp.push(':');
            base64::encode_config_buf(digest, base64::URL_SAFE, &mut tmp);

            tmp
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_sign() {
        let x = QiniuSigner::new("MY_ACCESS_KEY", "MY_SECRET_KEY");

        let url = "https://rs.qiniu.com/move/bmV3ZG9jczpmaW5kX21hbi50eHQ=/bmV3ZG9jczpmaW5kLm1hbi50eHQ=";
        let url = url::Url::parse(url).unwrap();
        assert_eq!(x.sign(&url, None), "MY_ACCESS_KEY:FXsYh0wKHYPEsIAgdPD9OfjkeEM=");
    }
}
