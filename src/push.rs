use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use super::config;

#[derive(Debug, StructOpt)]
pub struct Push {}
impl Push {
    pub fn run(&self, config_path: PathBuf, url: String) -> Result<String, String> {
        let file = File::open(config_path).expect("Could not open file.");
        let config: config::Config = serde_yaml::from_reader(file).expect("Could not read values.");

        //loop through the topics and create the schemas
        for topic in config.topics {
            let topic_name = format!("{}.{}.{}", config.service, topic.resource, topic.purpose);
            println!("{:?}", topic_name);
        }

        Ok("Success".to_string())
    }
}
