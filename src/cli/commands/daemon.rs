use crate::{
    cli::commands::run,
    platform::{ColorModeDaemon, ColorModeDetector},
};

pub async fn daemon<F, Futu, Daemon>(native_adapter: F, verbose: bool) -> anyhow::Result<()>
where
    Futu: std::future::Future<Output = anyhow::Result<Daemon>>,
    F: FnOnce() -> Futu,
    Daemon: ColorModeDaemon + ColorModeDetector,
{
    let adapter = native_adapter().await?;
    println!("😈 Running scripts initially for current color mode...");
    // FIXME: Actually handle errors here
    let mode = adapter.current_mode().await.unwrap();
    run(mode, verbose, true);

    println!("😈 Spawning daemon...");
    adapter
        .on_color_changed(move |mode| run(mode, verbose, true))
        .await;
    Ok(())
}
