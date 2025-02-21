#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "rust_webapp_boilerplate=trace,webapp_embedded_server=trace");
    env_logger::init();

    //TODO: use clap args?
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

    // start server
    println!("starting server at {}", bind_address);
    webapp_embedded_server::start(bind_address).await?;

    Ok(())
}
