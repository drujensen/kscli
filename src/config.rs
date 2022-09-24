use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub retry: bool,
    pub dlt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TopicType {
    request,
    reply,
    event,
    store,
    log,
}
impl fmt::Display for TopicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub resource: String,
    pub purpose: TopicType,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub service: String,
    pub schema_path: String,
    pub topics: Vec<Topic>,
}
