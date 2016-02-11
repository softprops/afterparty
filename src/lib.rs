#[macro_use]
extern crate log;
#[macro_use]
extern crate hyper;
extern crate crypto;
extern crate rustc_serialize;

mod hook;
mod events;

pub use events::Event;
pub use hook::{AuthenticateHook, Hook};
use hyper::server::{Handler, Request, Response};
use rustc_serialize::json;
use std::collections::HashMap;
use std::io::Read;

/// signature for request
/// see [this document](https://developer.github.com/webhooks/securing/) for more information
header! {(XHubSignature, "X-Hub-Signature") => [String]}

/// name of Github event
/// see [this document](https://developer.github.com/webhooks/#events) for available types
header! {(XGithubEvent, "X-Github-Event") => [String]}

/// unique id for each delivery
header! {(XGithubDelivery, "X-Github-Delivery") => [String]}

/// A delivery encodes all information about web hook request
//#[derive(Default)] default Event?
pub struct Delivery<'a> {
    pub id: &'a str,
    pub event: &'a str,
    pub payload: Event,
    pub unparsed_payload: &'a str,
    pub signature: Option<&'a str>,
}

/// A hub is a handler for github event requests
#[derive(Default)]
pub struct Hub {
    hooks: HashMap<String, Vec<Box<Hook>>>,
}

impl Hub {
    /// construct a new hub instance
    pub fn new() -> Hub {
        Hub { ..Default::default() }
    }

    pub fn authenticated<H, S>(&mut self, event: &str, secret: S, hook: H)
        where H: Hook + 'static,
              S: Into<String>
    {
        self.add(event, AuthenticateHook::new(secret, hook))
    }

    /// add a need hook registered to event type
    pub fn add<H>(&mut self, event: &str, hook: H)
        where H: Hook + 'static
    {
        self.hooks
            .entry(event.to_owned())
            .or_insert(vec![])
            .push(Box::new(hook));
    }

    /// get all interested hooks for a given event
    pub fn hooks(&self, event: &str) -> Option<Vec<&Box<Hook>>> {
        let explicit = self.hooks.get(event);
        let implicit = self.hooks.get("*");
        let combined = match (explicit, implicit) {
            (Some(ex), Some(im)) => {
                Some(ex.iter().chain(im.iter()).into_iter().collect::<Vec<_>>())
            }
            (Some(ex), _) => Some(ex.into_iter().collect::<Vec<_>>()),
            (_, Some(im)) => Some(im.into_iter().collect::<Vec<_>>()),
            _ => None,
        };
        combined

    }
}

impl Handler for Hub {
    fn handle(&self, mut req: Request, res: Response) {
        let mut payload = String::new();
        let headers = req.headers.clone();
        if let (Some(&XGithubEvent(ref event)),
                Some(&XGithubDelivery(ref delivery))) = (headers.get::<XGithubEvent>(),
                                                         headers.get::<XGithubDelivery>()) {
            if let Ok(_) = req.read_to_string(&mut payload) {
                match json::decode::<Event>(&payload) {
                    Ok(parsed) => {
                        let signature = headers.get::<XHubSignature>();
                        info!("recv '{}' event with signature '{:?}'", event, signature);
                        let delivery = Delivery {
                            id: delivery,
                            event: event,
                            payload: parsed,
                            unparsed_payload: payload.as_ref(),
                            signature: signature.map(|s| s.as_ref()),
                        };
                        if let Some(hooks) = self.hooks(event) {
                            for hook in hooks {
                                hook.handle(&delivery)
                            }
                        }
                    }
                    _ => {
                        error!("failed to parse event {:?} for delivery {:?}",
                               event,
                               delivery)
                    }
                }
            }
        }
        let _ = res.send(b"ok");
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hub_hooks() {
        let mut hub = Hub::new();
        hub.add("push", |_: &Delivery| {});
        hub.add("*", |_: &Delivery| {});
        assert_eq!(Some(2),
                   hub.hooks("push").map(|hooks| hooks.into_iter().count()))
    }
}
