#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub service: String,
    pub schema_path: String,
    pub topics: Vec<Topic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub purpose: TopicType,
    pub resource: String,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TopicType {
    request,
    reply,
    event,
    store,
    log,
}
impl Display for TopicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub compatibility: Compatibility,
    pub retry: isize,
    pub dlt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Compatibility {
    BACKWARD,
    BACKWARD_TRANSITIVE,
    FORWARD,
    FORWARD_TRANSITIVE,
    FULL,
    FULL_TRANSITIVE,
    NONE,
}
impl Display for Compatibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
