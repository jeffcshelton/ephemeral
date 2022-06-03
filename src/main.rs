use {
	clap::{Parser, Subcommand},
	rand::Rng,
};

const DEFAULT_CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890~!@#$%^&*()-_=+[]{}:;<>?";

#[derive(Subcommand)]
enum Action {
	Gen {
		#[clap(default_value_t = 1)]
		count: usize,

		#[clap(long("len"), default_value_t = 16)]
		length: usize,

		#[clap(long("charset"))]
		character_set: Option<String>,

		#[clap(long("no-caps"))]
		no_capitals: bool,

		#[clap(long("no-nums"))]
		no_numbers: bool,

		#[clap(long("no-symbols"))]
		no_symbols: bool,
	}
}

#[derive(Parser)]
struct Args {
	#[clap(subcommand)]
	action: Action
}

fn main() {
	let args = Args::parse();

	match args.action {
		Action::Gen { count, length, character_set, no_capitals, no_numbers, no_symbols } => {
			let mut charset = DEFAULT_CHARSET.to_owned();

			if let Some(character_set) = character_set {
				charset = character_set;
			} else {
				if no_symbols {
					charset.replace_range(62.., "");
				}

				if no_numbers {
					charset.replace_range(52..62, "");
				}

				if no_capitals {
					charset.replace_range(..26, "");
				}
			}

			let mut rng = rand::thread_rng();

			for _ in 0..count {
				let mut password = String::with_capacity(length);

				for _ in 0..length {
					let index = rng.gen_range(0..charset.len());

					password.push(
						charset
							.chars()
							.nth(index)
							.unwrap()
					);
				}

				println!("{}", password);
			}
		}
	}
}
