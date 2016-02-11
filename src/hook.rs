use rustc_serialize::hex::ToHex;
use super::Delivery;
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;

pub trait Hook: Send + Sync {
    fn handle(&self, delivery: &Delivery);
}

// and authenticator for hooks
pub struct AuthenticateHook<H: Hook + 'static> {
    secret: String,
    hook: H,
}

impl<H: Hook + 'static> AuthenticateHook<H> {
    pub fn new<S>(secret: S, hook: H) -> AuthenticateHook<H>
        where S: Into<String>
    {
        AuthenticateHook {
            secret: secret.into(),
            hook: hook,
        }
    }

    fn authenticate(&self, payload: &str, signature: &str) -> bool {
        let sbytes = self.secret.as_bytes();
        let pbytes = payload.as_bytes();
        let mut mac = Hmac::new(Sha1::new(), &sbytes);
        mac.input(&pbytes);
        mac.result().code().to_hex() == str::replace(signature, "sha1=", "")
    }
}

impl<H: Hook + 'static> Hook for AuthenticateHook<H> {
    fn handle(&self, delivery: &Delivery) {
        if let Some(sig) = delivery.signature {
            if self.authenticate(delivery.unparsed_payload, sig) {
                self.hook.handle(delivery)
            }
        }
    }
}

// fn hook
impl<F> Hook for F
    where F: Fn(&Delivery),
          F: Sync + Send
{
    fn handle(&self, delivery: &Delivery) {
        self(delivery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Delivery;
    use crypto::mac::Mac;
    use crypto::hmac::Hmac;
    use crypto::sha1::Sha1;
    use rustc_serialize::hex::ToHex;

    #[test]
    fn authenticate_signatures() {
        let authenticated = AuthenticateHook::new("secret", |_: &Delivery| {
        });
        let payload = r#"{"zen": "Approachable is better than simple."}"#;
        let secret = "secret";
        let sbytes = secret.as_bytes();
        let pbytes = payload.as_bytes();
        let mut mac = Hmac::new(Sha1::new(), &sbytes);
        mac.input(&pbytes);
        let signature = mac.result().code().to_hex();
        assert!(authenticated.authenticate(payload, format!("sha1={}", signature).as_ref()))
    }
}
