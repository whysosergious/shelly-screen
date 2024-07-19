use clap::Parser;
use futures::TryStreamExt;
use colored::*;
use tokio;
use brightness::Brightness;


use std::process::Command;

use execute::Execute;

pub enum BrightMode {
    Increase,
    Decrease,
}

#[derive(Parser, Clone)] 
struct Cli {
    #[clap(subcommand)]
    command: Commands,
       #[clap(short, long, default_value = "true")]
    quiet: bool,
    #[clap(short, long, default_value = "10")]
    step: u32
}

#[derive(Parser, Clone)] 
enum Commands {
    Set(Args),
    Inc(SetArgs),
    Dec(DecArgs),
    Max,
    Min,
    Get,
    Step(Args),
Tq,
}

 #[derive(Parser, Clone)]
 struct SetArgs {
 #[arg(value_parser = clap::value_parser!(u32).range(0..=100))]
     percent: Option<u32>,
 }

#[derive(Parser,    Clone)]
struct Args {
 #[arg(value_parser = clap::value_parser!(u32).range(0..=100))]
    percent: u32,
}

#[derive(Parser, Clone)]
struct DecArgs {
    percent: Option<u32>,
}



fn exec(value: String) -> String {

let mut command = Command::new("nu");
command.arg(format!("-e '$env.scq = {}'", value));
let output = command.execute_output().unwrap();
    println!("{:?}", output);
    return output
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut cli = Cli::parse();

exec(cli.quiet.to_string());
       match &cli.command {





        Commands::Set(args) => {
             let percent = args.percent;
            let mut devices = brightness::brightness_devices();
            while let Some(mut device) = devices.try_next().await? {
                device.set(percent).await?;
            }       
        }
        Commands::Inc(args) => {
  let percent = args.percent.unwrap_or(cli.step.into());
            let mut devices = brightness::brightness_devices();
            while let Some(mut device) = devices.try_next().await? {
                let level = device.get().await?;
                device.set(level + percent).await?;           
            }
        }
        Commands::Dec(args) => {
  let percent = args.percent.unwrap_or(cli.step.into());
            let mut devices = brightness::brightness_devices();
            while let Some(mut device) = devices.try_next().await? {
                let level = device.get().await?;
                device.set(level - percent).await?;                     
            }
        }
        Commands::Max => {
            let mut devices = brightness::brightness_devices();
            while let Some(mut device) = devices.try_next().await? {
                device.set(100).await?;
            }
        }
        Commands::Min => {
            let mut devices = brightness::brightness_devices();
            while let Some(mut device) = devices.try_next().await? {
                device.set(0).await?;
            }
        }
        Commands::Get => {
            print_value().await?;
        }
        Commands::Step (args)=> {
         
                                      cli.step = args.percent;
                println!("{} {}", "step set to", args.percent);
                   
        }

        Commands::Tq => {
            cli.quiet = !cli.quiet;
        }
    }

    if !cli.quiet{
        print_value().await?;
    }

    Ok(())
}

async fn print_value() -> Result<(), Box<dyn std::error::Error>> {
    brightness::brightness_devices()
        .try_for_each(|device| async move {
            let result = device.get().await?;
            println!("{}", format!("{}", result).cyan().bold());
            Ok(())
        })
        .await?;
    Ok(())
}
