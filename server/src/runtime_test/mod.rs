use clap::Parser;

#[derive(Debug, Parser)]
pub struct Test {}

#[allow(unused_variables)]
pub async fn test(args: Test) {
	eprintln!("Test command executed with args: {args:?}");

	// !!! do not commit this code !!!
}
