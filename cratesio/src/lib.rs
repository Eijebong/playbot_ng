extern crate reqwest;
extern crate url;
#[macro_use] extern crate serde_derive;

use reqwest::r#async as async_reqwest;
use futures::prelude::*;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};

pub fn crate_info(name: &str) -> Result<Info, reqwest::Error> {
    let url = format!(
        "https://crates.io/api/v1/crates/{}",
        utf8_percent_encode(name, PATH_SEGMENT_ENCODE_SET).collect::<String>()
    );
    let info = reqwest::get(&url)?
        .json()?;

    Ok(info)
}

pub fn async_crate_info(name: &str) -> impl Future<Item = Info, Error = reqwest::Error> {
    let client = async_reqwest::Client::new();
    let url = format!(
        "https://crates.io/api/v1/crates/{}",
        utf8_percent_encode(name, PATH_SEGMENT_ENCODE_SET).collect::<String>()
    );

    client
    .get(&url)
    .send()
    .and_then(|mut resp| resp.json())
}

#[derive(Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct Info {
    #[serde(rename = "crate")]
    krate: Crate,
}

#[derive(Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct Crate {
    id: String,
    name: String,
    description: String,
    max_version: String,
}

impl Info {
    pub fn krate(&self) -> &Crate {
        &self.krate
    }
}

impl Crate {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn max_version(&self) -> &str {
        &self.max_version
    }
}
