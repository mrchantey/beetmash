#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
	/* */
	beetmash_server::server::Server::default().run().await
}
