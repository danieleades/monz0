use structopt::StructOpt;
use monzo::Client;
use monzo::Result;


#[derive(StructOpt)]
pub struct Command {
}

impl Command {
    pub async fn run(&self, client: &Client) -> Result<()> {
        let accounts = client.accounts().send().await?;

        println!("accounts:\n\n{:#?}", accounts);

        Ok(())
    }
}