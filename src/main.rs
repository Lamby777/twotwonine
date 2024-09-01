use clap::Parser;

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

fn encode(data: &str) {
    todo!()
}

fn decode(data: &str) {
    todo!()
}
