use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Pull {}
impl Pull {
    pub fn run(&self, config: PathBuf, url: String) -> Result<String, String> {
        println!("Currently not implemented");
        Ok("Success".to_string())
    }
}
