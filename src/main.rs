use kountr_web::{Server, AppOptions, dotenv};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let opts = AppOptions::new_from_envs();

    Server::start(&opts).await
}
