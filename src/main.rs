use {
	clap::{Parser, Subcommand}
};

#[derive(Subcommand)]
enum Action {
	Gen {
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
}
