use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(skip_serializing_if = "Option::is_none")]
pub struct DomainList {
  pub CNAME_speedup: String,
  pub domain_id: u32,
  pub name: String,
}