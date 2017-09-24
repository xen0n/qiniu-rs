use base64;
use ring;


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

    pub fn sign<S: AsRef<[u8]>, T: AsRef<[u8]>>(&'a self, path: S, body: T) -> String {
        let mut ctx = ring::hmac::SigningContext::with_key(&self.sk);
        ctx.update(path.as_ref());
        ctx.update(b"\n");
        ctx.update(body.as_ref());

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

        let path = "/move/bmV3ZG9jczpmaW5kX21hbi50eHQ=/bmV3ZG9jczpmaW5kLm1hbi50eHQ=";
        assert_eq!(x.sign(path, b""), "MY_ACCESS_KEY:FXsYh0wKHYPEsIAgdPD9OfjkeEM=");
    }
}
