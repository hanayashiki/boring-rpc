mod args;
mod run;

fn main() -> anyhow::Result<()> {
    run::try_run(std::env::args_os())
}
