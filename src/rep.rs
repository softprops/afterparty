//! A verify minimal representation of payload information

#[derive(Debug, RustcEncodable, RustcDecodable, Default)]
pub struct Payload {
  pub sender: Sender,
  pub organization: Option<Organization>,
  pub repository: Option<Repository>
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct Sender {
  pub login: String
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct Organization {
  pub login: String
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct Repository {
  pub name: String,
  pub full_name: String,
  pub organization: Organization
}