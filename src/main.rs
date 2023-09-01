use kountr_web::App;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::start().await
}
