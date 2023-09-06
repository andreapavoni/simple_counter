use kountr_web::Web;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Web::start().await
}
