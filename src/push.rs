use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

use super::config;
use super::utils;

#[derive(Debug, StructOpt)]
pub struct Push {}
impl Push {
    pub fn run(&self, config_path: PathBuf, url: String) -> Result<String, String> {
        let file = File::open(config_path).expect("Could not open file.");
        let config: config::Config = serde_yaml::from_reader(file).expect("Could not read values.");

        for topic in config.topics {
            let schema_asvc = format!(
                "{}/{}-{}.avsc",
                config.schema_path, topic.resource, topic.purpose
            );
            let mut asvc = File::open(schema_asvc).expect("Could not open asvc file.");
            let mut buffer = String::new();
            asvc.read_to_string(&mut buffer)
                .expect("Could not read file.");
            let body = format!("{{\"schema\": \"{}\"}}", utils::escape(&buffer));

            let client = Client::new();
            let topic_name = format!("{}.{}.{}", topic.purpose, config.service, topic.resource);
            let subject = format!("{}/subjects/{}-value/versions", url, topic_name);
            let response = client
                .post(subject)
                .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
                .body(body.clone())
                .send()
                .expect("Could not send request.");
            println!("{:?}", response.text());

            if topic.properties.retry {
                let subject = format!("{}/subjects/{}-retry-0-value/versions", url, topic_name);

                let response = client
                    .post(subject)
                    .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
                    .body(body.clone())
                    .send()
                    .expect("Could not send request.");
                println!("{:?}", response.text());
            }

            if topic.properties.dlt {
                let schema_asvc = format!(
                    "{}/{}-{}.avsc",
                    config.schema_path, topic.resource, topic.purpose
                );
                let mut asvc = File::open(schema_asvc).expect("Could not open asvc file.");
                let subject = format!("{}/subjects/{}-dlt-value/versions", url, topic_name);
                let mut buffer = String::new();
                asvc.read_to_string(&mut buffer)
                    .expect("Could not read file.");
                let body = format!("{{\"schema\": \"{}\"}}", utils::escape(&buffer));

                let response = client
                    .post(subject)
                    .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
                    .body(body.clone())
                    .send()
                    .expect("Could not send request.");
                println!("{:?}", response.text());

                let subject = format!("{}/config/{}-dlt-value", url, topic_name);
                let response = client
                    .put(subject)
                    .header(CONTENT_TYPE, "application/vnd.schemaregistry.v1+json")
                    .body("{\"compatibility\": \"NONE\"}")
                    .send()
                    .expect("Could not send request.");
                println!("{:?}", response.text());
            }
        }

        Ok("Success".to_string())
    }
}
