use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use super::config;

#[derive(Debug, StructOpt)]
pub struct Init {}
impl Init {
    pub fn run(&self, config_path: PathBuf) -> Result<String, String> {
        let request_topic = config::Topic {
            resource: "post".to_string(),
            purpose: config::TopicType::request,
            properties: config::Properties {
                compatibility: config::Compatibility::FULL,
                retry: 1,
                dlt: true,
            },
        };

        let config = config::Config {
            service: "blog".to_string(),
            schema_path: "./schemas".to_string(),
            topics: vec![request_topic],
        };

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).expect("Couldn't create directory");
        };

        let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(config_path.clone())
            .expect("Couldn't open file");

        serde_yaml::to_writer(file, &config).unwrap();

        // Create sample schema
        let schema = r#"{
    "namespace": "com.example.blog",
    "type": "record",
    "name": "PostRequest",
    "fields": [
        {"name": "id", "type": "string"},
        {"name": "title", "type": "string"},
        {"name": "body", "type": "string"}
    ]
}"#;

        let path: PathBuf = [&config.schema_path, "request", "post.avsc"]
            .iter()
            .collect();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Couldn't create directory");
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("Couldn't open file");

        write!(file, "{}", schema).unwrap();

        Ok("Success".to_string())
    }
}
