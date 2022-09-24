use std::path::PathBuf;
use structopt::StructOpt;

use super::config;

#[derive(Debug, StructOpt)]
pub struct Init {}
impl Init {
    pub fn run(&self, config_path: PathBuf, _url: String) -> Result<String, String> {
        let request_topic = config::Topic {
            resource: "post".to_string(),
            purpose: config::TopicType::request,
            properties: config::Properties {
                retry: true,
                dlt: true,
            },
        };

        let reply_topic = config::Topic {
            resource: "post".to_string(),
            purpose: config::TopicType::reply,
            properties: config::Properties {
                retry: true,
                dlt: true,
            },
        };

        let event_topic = config::Topic {
            resource: "post".to_string(),
            purpose: config::TopicType::event,
            properties: config::Properties {
                retry: true,
                dlt: true,
            },
        };

        let config = config::Config {
            service: "blog".to_string(),
            schema_path: "./schemas".to_string(),
            topics: vec![request_topic, reply_topic, event_topic],
        };

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)
            .expect("Couldn't open file");

        serde_yaml::to_writer(file, &config).unwrap();

        Ok("Success".to_string())
    }
}
