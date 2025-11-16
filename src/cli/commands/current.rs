use crate::platform::{ColorModeDaemon, ColorModeDetector};

pub async fn current<F, Futu, Daemon>(
    native_adapter: F,
    watch: bool,
    plain: bool,
) -> anyhow::Result<()>
where
    Futu: std::future::Future<Output = anyhow::Result<Daemon>>,
    F: FnOnce() -> Futu,
    Daemon: ColorModeDaemon + ColorModeDetector,
{
    // FIXME: error handling
    let adapter = native_adapter().await?;
    let mode = adapter.current_mode().await.unwrap();
    if plain {
        println!("{}", mode);
    } else {
        println!("{} {}", mode.emoji(), mode);
    }
    if !watch {
        return Ok(());
    }

    adapter
        .on_color_changed(move |mode| {
            if plain {
                println!("{}", mode);
            } else {
                println!("{} {}", mode.emoji(), mode);
            }
        })
        .await;
    Ok(())
}
