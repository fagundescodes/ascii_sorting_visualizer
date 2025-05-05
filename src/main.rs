use color_eyre::Result;

mod app;
mod sort;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = app::App::new();
    app.run_ui()?;

    Ok(())
}
