//! A minimal representation of payload information

#[derive(Debug, RustcDecodable, Default)]
pub struct Ping {
  pub hook: Payload
}

#[derive(Debug, RustcDecodable, Default)]
pub struct Payload {
  pub sender: Sender,
  pub organization: Option<Organization>,
  pub repository: Option<Repository>
}

#[derive(Debug, Default, RustcDecodable)]
pub struct Sender {
  pub login: String
}

#[derive(Debug, Default, RustcDecodable)]
pub struct Organization {
  pub login: String
}

#[derive(Debug, Default, RustcDecodable)]
pub struct Repository {
  pub name: String,
  pub full_name: String,
  pub organization: Option<Organization>
}
