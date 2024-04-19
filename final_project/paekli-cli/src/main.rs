use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    Send { content: String },
    Receive,
}

/// send and receive pure distilled awesomeness 🍸
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
        .context("the user's home directory seems to be corrupt")?;
    let storage_dir = project_dir.data_dir();

    std::fs::create_dir_all(storage_dir).context("failed to create storage directory")?;
    match args.command {
        Command::Send { content } => {
            if storage_dir.join("content").as_path().exists() {
                anyhow::bail!("Storage is full!")
            }
            std::fs::write(storage_dir.join("content"), content)
                .context("failed to store paekli")?;
        }
        Command::Receive => {
            let content_vec =
                std::fs::read(storage_dir.join("content")).context("Failed to read paekli");
            match content_vec {
                Ok(content) => {
                    println!(
                        "{:?}",
                        String::from_utf8(content).context("Our bytes should be valid utf8")
                    );
                    let result = std::fs::remove_file(storage_dir.join("content"))
                        .context("could not delete file");
                    match result {
                        Ok(_) => println!("Freed up space"),
                        Err(_) => println!("Error freeing up space!"),
                    }
                }
                Err(_) => println!("Nothing to see here, please move on!"),
            };
        }
    }

    Ok(())
}
