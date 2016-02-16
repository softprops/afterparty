use super::Delivery;
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;

// fixme: borrowed from rustc, may exist somewhere in serde
static CHARS: &'static[u8] = b"0123456789abcdef";
pub fn to_hex(bs: &[u8]) -> String {
    let mut v = Vec::with_capacity(bs.len() * 2);
    for &byte in bs.iter() {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

/// Handles webhook deliveries
pub trait Hook: Send + Sync {
    /// Implementations are expected to deliveries here
    fn handle(&self, delivery: &Delivery);
}

/// A delivery authenticator for hooks
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
        to_hex(mac.result().code()) == str::replace(signature, "sha1=", "")
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
        let signature = to_hex(mac.result().code());
        assert!(authenticated.authenticate(payload, format!("sha1={}", signature).as_ref()))
    }
}
