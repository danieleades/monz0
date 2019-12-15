use structopt::StructOpt;
use monzo::{Client, Error, Result};

mod configuration;
mod app;

use configuration::Configuration;
use app::Command;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Configuration::load().unwrap();
    let (mut client, _app_config) = config.into_parts();
    let command = Command::from_args();
    run_or_refresh(&mut client, &command).await
}

async fn run_or_refresh(client: &mut Client, command: &Command) -> Result<()> {
    let result = command.run(client).await;

    match result {
        Err(Error::AuthExpired) => {
            command.run(client).await
        },
        a => a,
    }
}
