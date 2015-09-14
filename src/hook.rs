use time;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::prelude::*;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use super::Event;
use super::rep::Payload;

/// Represents a description of a github hook
/// A hook should describe a cmd to run and for which
/// events. It should be assumed that the github event name
/// and event payload will be available in env vars
/// `GH_EVENT_NAME` and `GH_EVENT_PAYLOAD` respectively
#[derive(Clone, Default, Debug, RustcDecodable, RustcEncodable)]
pub struct Hook {
  /// Command to run when an event is recieved
  pub cmd: String,
  /// List of github events that should trigger this hook
  pub events: Vec<String>,
  /// Optional name of hook. Defaults to cmd
  pub name: Option<String>,
  /// Github organization filter
  pub organization: Option<String>,
  /// Github repository filter
  pub repository: Option<String>,
  /// Github event sender filter
  pub sender: Option<String>
}

impl Hook {

  /// return a debug hook that targets all events
  /// and does nothing but echo the event name and payload
  pub fn echo() -> Hook {
     Hook {
       cmd: "echo \"$GH_EVENT_NAME\": \"$GH_EVENT_PAYLOAD\"".to_owned(),
       events: vec!["*".to_owned()],
       ..Default::default()
     }
  }

  fn info<S: Into<String>>(&self, delivery: &String, msg: S) {
    info!("{} {} {}: {}", time::now().to_utc().rfc3339(), delivery, self.name(), msg.into());
  }

  fn err<S: Into<String>>(&self, delivery: &String, msg: S) {
    error!("{} {} {}: {}", time::now().to_utc().rfc3339(), delivery, self.name(), msg.into())
  }


  /// return true if this hook targets the provided
  /// event name, false otherwise
  pub fn targets(&self, event: &String, payload: &Payload) -> bool {
    // todo: filter on payload.{ repo, org, sender }
    let targets_event = self.events.contains(event)
      || self.events.contains(&"*".to_owned());
    if let Some(ref porg) = payload.organization {
      if let Some(ref org) = self.organization {
        if porg.login != *org {
          return false
        }
      }
    }
    if let Some(ref prepo) = payload.repository {
      if let Some(ref repo) = self.repository {
        if prepo.name != *repo {
          return false
        }
      }
    }
    targets_event
  }

  /// return name if defined, otherwise cmd
  pub fn name(&self) -> String {
    self.name.clone().unwrap_or(self.cmd.clone())
  }

  /// start recv'ing events
  pub fn recv(&self, rx: Receiver<Event>) {
    loop {
      match rx.recv() {
        Ok(event) => {
          self.run(&event);
          ()
        },
        Err(e) => {
          self.err(&"???".to_owned(), format!("Recv err: {}\n", e.to_string()));
          break
        }
      }
    }
  }

  /// run hook cmd, returning true if cmd succeeded, otherwise false
  pub fn run(&self, event: &Event) -> bool {

     fn collect<T: Read + Send + 'static>(stm: Option<T>) ->
       Receiver<io::Result<Vec<String>>> {
       let (tx, rx) = channel();
       if let Some(s) = stm {
         thread::spawn(move || {
           let mut buf = BufReader::with_capacity(64, s);
           let mut lines: Vec<String> = Vec::new();
           loop {
             let mut line = String::new();
             match buf.read_line(&mut line) {
                Ok(0) | Err(_)  => break,
                Ok(_)  => lines.push(line)
             }
           }
           tx.send(Ok(lines)).unwrap()
         });
       } else {
         tx.send(Ok(vec![])).unwrap();
       }
       rx
     }

     match Command::new("/bin/sh")
       .arg("-c")
       .arg(&self.cmd)
       .env("GH_DELIVERY", &event.delivery)
       .env("GH_EVENT_NAME", &event.name)
       .env("GH_EVENT_PAYLOAD", &event.payload)
       .stdin(Stdio::null())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped())
       .spawn() {
         Err(e) => {
           self.err(&event.delivery, format!("error executing {}: {}\n", self.cmd, e));
           false
         },
         Ok(mut child)  => {

           let stdout = collect(child.stdout.take());
           let stderr = collect(child.stderr.take());
           let status = child.wait();

           match stdout.recv() {
             Ok(Ok(lines)) => for l in lines {
               self.info(&event.delivery, l);
             },
             Ok(Err(e))    => self.err(&event.delivery, format!("stdout io err {}\n", e)),
             Err(e)        => self.err(&event.delivery, format!("stdout recv err {}\n", e))
           };

           match stderr.recv() {
             Ok(Ok(lines)) => for l in lines {
               self.info(&event.delivery, l)
             },
             Ok(Err(e))    => self.err(&event.delivery, format!("stderr io err {}\n", e)),
             Err(e)        => self.err(&event.delivery, format!("stderr recv err {}\n", e))
           };

           match status {
             Ok(s) => {
               if s.success() {
                 self.info(&event.delivery, "that worked\n".to_owned());
                 true
               } else {
                 match s.code() {
                   Some(c) => self.info(&event.delivery, format!("hook exited with status {}\n", c)),
                   _ => if let Some(s) = s.signal() {
                     self.err(&event.delivery, format!("process killed by signal {}\n", s))
                   }
                 };
                 false
               }
             },
             Err(e) => {
               self.err(&event.delivery, format!("error getting exit status {}\n", e.to_string()));
               false
             }
           }
         }
       }
  }
}

#[cfg(test)]
mod tests {
  use super::Hook;
  use super::super::Event;

  #[test]
  fn test_hook_run_pass() {
    let hook = Hook {
      cmd: "true".to_owned(),
      ..Default::default()
    };
    assert!(hook.run(&Event {
      ..Default::default()
    }))
  }

  #[test]
  fn test_hook_run_fail() {
    let hook = Hook {
      cmd: "false".to_owned(),
      ..Default::default()
    };
    assert!(!hook.run(&Event {
      ..Default::default()
    }))
  }

  #[test]
  fn test_hook_run_panic() {
    let hook = Hook {
      cmd: "oiusdfasdf".to_owned(),
      ..Default::default()
    };
    assert!(!hook.run(&Event {
      ..Default::default()
    }))
  }
}
