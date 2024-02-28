use volt::Volt;

#[tokio::main]
async fn main() {
	Volt::run().await.expect("Couldnt start");
}
