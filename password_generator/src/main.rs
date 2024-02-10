use clap::Parser;
use rand::thread_rng;
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
const SYMBOLS_CHARSET: &[u8] = b")(*&^%$#@!~?><,.;:}{][=";

#[derive(Parser)]
#[command(version, author = "Raunak Raj <bajrangcoders@gmail.com>", about = "A strong password generator cli", long_about = None)]
struct Cli {
    /// length for password
    length: usize,
    /// include symbols
    #[arg(short, long)]
    symbols: bool,
}

fn main() {
    let cli: Cli = Cli::parse();
    let mut rng = thread_rng();
    let password: String = if cli.symbols {
        let new_charset = [CHARSET, SYMBOLS_CHARSET].concat();
        (0..cli.length)
            .map(|_| {
                let idx = rng.gen_range(0..new_charset.len());
                new_charset[idx] as char
            })
            .collect()
    } else {
        (0..cli.length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    };
    println!("🔒 Strong password : {}", password);
}
