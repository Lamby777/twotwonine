use clap::Parser;
use color_space::{Hsv, Rgb};

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

fn encode(data: &str) -> String {
    let ascii = data.chars().map(|c| c as u8).collect::<Vec<_>>();
    let rgbs = ascii
        .chunks(3)
        .map(|chunk| {
            let r = chunk.get(0).unwrap_or(&32);
            let g = chunk.get(1).unwrap_or(&32);
            let b = chunk.get(2).unwrap_or(&32);
            Rgb::new(*r as f64, *g as f64, *b as f64)
        })
        .collect::<Vec<_>>();

    rgbs.iter()
        .map(|rgb| Hsv::from(*rgb))
        .map(|hsv| format!("{:.1} {:.1} {:.1}", hsv.h, (hsv.s * 100.0), (hsv.v * 100.0)))
        .collect()
}

fn decode(data: &str) -> String {
    let hsvs = data
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let rgbs = hsvs
        .chunks(3)
        .map(|chunk| Hsv::new(chunk[0], chunk[1] / 100.0, chunk[2] / 100.0))
        .map(|hsv| Rgb::from(hsv))
        .collect::<Vec<_>>();

    let ascii = rgbs
        .iter()
        .map(|rgb| {
            vec![
                rgb.r.round() as u8,
                rgb.g.round() as u8,
                rgb.b.round() as u8,
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    ascii.iter().map(|&c| c as char).collect()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn encode_ILY() {
        assert_eq!(encode("ILY"), "228.8 18.0 34.9")
    }

    #[test]
    fn encode_ily() {
        assert_eq!(encode("ily"), "228.8 13.2 47.5")
    }

    #[test]
    fn decode_2288_180_349() {
        assert_eq!(decode("228.8 18.0 34.9"), "ILY")
    }

    #[test]
    fn decode_2288_132_475() {
        assert_eq!(decode("228.8 13.2 47.5"), "ily")
    }
}
