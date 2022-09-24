use std::path::PathBuf;
use structopt::StructOpt;

mod config;
mod init;
mod pull;
mod push;
mod utils;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "init", about = "Initialize a new configuration file")]
    Init(init::Init),

    #[structopt(name = "push", about = "Push the schemas to the registry")]
    Push(push::Push),

    #[structopt(name = "pull", about = "Pull the schemas from the registry")]
    Pull(pull::Pull),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kscli", about = "Kafka Schema Registry CLI")]
struct Opt {
    #[structopt(
        short,
        long,
        env = "KAFKA_SCHEMA_CLI_CONFIG",
        default_value = "config/ksconfig.yml"
    )]
    config: PathBuf,

    #[structopt(
        short,
        long,
        env = "KAFKA_SCHEMA_REGISTRY_URL",
        default_value = "http://localhost:8081"
    )]
    url: String,

    #[structopt(subcommand)]
    pub command: Command,
}

fn main() {
    let opt = Opt::from_args();

    let result = match opt.command {
        Command::Init(init) => init.run(opt.config, opt.url),
        Command::Push(push) => push.run(opt.config, opt.url),
        Command::Pull(pull) => pull.run(opt.config, opt.url),
    };

    println!("{:#?}", result.unwrap());
}
