mod app;
mod controller;
mod request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let controller = controller::Controller::new();

    app::run()?;

    controller.join().await;

    Ok(())
}
