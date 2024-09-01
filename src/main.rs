use clap::Parser;
use colors_transform::{Color, Rgb};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    decode: bool,

    data: String,
}

fn main() {
    let args = Args::parse();

    match args.decode {
        true => {
            eprintln!("Decoding...");
            decode(&args.data);
        }

        false => {
            eprintln!("Encoding...");
            encode(&args.data);
        }
    }
}

fn tuple3_to_str(tuple: (f32, f32, f32)) -> String {
    format!("{} {} {}", tuple.0, tuple.1, tuple.2)
}

fn encode(data: &str) -> String {
    let ascii = data.chars().map(|c| c as u8).collect::<Vec<_>>();
    let rgbs = ascii
        .chunks(3)
        .map(|chunk| {
            let r = chunk.get(0).unwrap_or(&32);
            let g = chunk.get(1).unwrap_or(&32);
            let b = chunk.get(2).unwrap_or(&32);
            Rgb::from(*r as f32, *g as f32, *b as f32)
        })
        .collect::<Vec<_>>();

    rgbs.iter()
        .map(|rgb| tuple3_to_str(rgb.to_hsl().as_tuple()))
        .collect()
}

fn decode(data: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ily() {
        assert_eq!(encode("ILY"), "229 18 35")
    }
}
