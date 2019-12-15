use structopt::StructOpt;
use structopt::clap::AppSettings;
use monzo::{Client, Result};

mod transactions;
mod accounts;
mod refresh;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
pub enum Command {
    /// Retrieve and manipulate transactions
    Transactions(transactions::Command),
    Accounts(accounts::Command),
}

impl Command {
    pub async fn run(&self, client: &Client) -> Result<()> {
        match self {
            Self::Transactions(command) => command.run(client).await,
            Self::Accounts(command) => command.run(client).await,
        }
    }
}