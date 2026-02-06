mod app;
mod terminal;
mod ui;
mod widgets;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "claudechic")]
#[command(about = "A stylish terminal UI for Claude Code", long_about = None)]
struct Args {
    #[arg(short, long)]
    resume: Option<String>,

    #[arg(short = 's', long)]
    session: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("claudechic=debug".parse()?),
        )
        .init();

    let args = Args::parse();

    println!("Claude Chic v0.1.0");
    println!("A stylish terminal UI for Claude Code");
    println!();

    if args.resume.is_some() || args.session.is_some() {
        println!("Session resume not yet implemented");
    } else {
        println!("Starting new session...");
    }

    println!("Core infrastructure initialized and ready!");
    println!();
    println!("Next steps:");
    println!("  - Implement database schema and migrations");
    println!("  - Build Terminal UI with Ratatui");
    println!("  - Integrate Claude Agent SDK");
    println!("  - Add multi-agent support");

    Ok(())
}
