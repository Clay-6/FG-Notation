mod cli;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use fg_notation::{abbreviated as a, numpad as n};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.target {
        cli::Target::Numpad { abbr } => {
            let moves = {
                let mut tmp = vec![];
                for m in abbr {
                    tmp.push(a::Move::new(m)?)
                }
                tmp
            };
            let converted = moves
                .iter()
                .map(|m| n::Move::from(m.clone()))
                .collect::<Vec<n::Move>>();
            print!("{}", converted[0]);
            for mv in converted.iter().skip(1) {
                print!(" -> {}", mv);
            }
        }
        cli::Target::Abbreviate { nump } => {
            let moves = {
                let mut tmp = vec![];
                for mv in nump {
                    tmp.push(n::Move::new(mv)?)
                }
                tmp
            };
            let converted = moves
                .iter()
                .map(|m| a::Move::from(m.clone()))
                .collect::<Vec<a::Move>>();
            print!["{}", converted[0]];
            for mv in converted.iter().skip(1) {
                print! {" -> {}", mv}
            }
        }
    }

    Ok(())
}
