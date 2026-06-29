mod app;
mod cli;
mod commands;
mod platform;
mod storage;

fn main() -> anyhow::Result<()> {
    app::run()
}
