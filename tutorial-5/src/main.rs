use tutorial_5::serve::server;
#[tokio::main]
pub async fn main() {
    // this func should be commented to avoid main func block ci/cd pipeline
    server::start_server().await
}
