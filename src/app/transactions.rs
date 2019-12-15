use structopt::StructOpt;
use monzo::Client;
use monzo::Result;


#[derive(StructOpt)]
pub enum Command {
    /// List transactions
    List(list::Command),
    /// Get a transaction, by ID
    Get(get::Command),
}

impl Command {
    pub async fn run(&self, client: &Client) -> Result<()> {
        match self {
            Self::List(command) => command.run(client).await,
            Self::Get(command) => command.run(client).await,
        }
    }
}

mod list {
    use structopt::StructOpt;
    use monzo::Client;
    use monzo::Result;
    
    #[derive(StructOpt)]
    pub struct Command {

        account_id: String,

        #[structopt(long, short, default_value = "15")]
        limit: u16
    }

    impl Command {
        pub async fn run(&self, client: &Client) -> Result<()> {
            let transactions = client.transactions(&self.account_id).limit(self.limit).send().await?;

            println!("{:#?}", transactions);
    
            Ok(())
        }
    }
}

mod get {
    use structopt::StructOpt;
    use monzo::Client;
    use monzo::Result;

    #[derive(StructOpt)]
    pub struct Command {
        transaction_id: String
    }

    impl Command {
        pub async fn run(&self, client: &Client) -> Result<()> {
            let transaction = client.transaction(&self.transaction_id).send().await?;

            println!("{:#?}", transaction);
    
            Ok(())
        }
    }
}