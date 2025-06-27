/*!
This is the application which runs on the server itself.

It can be used as cli and also runs a gprc server.

How to install this
*/

mod setup;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
	#[clap(subcommand)]
	subcmd: SubCommand,
	#[clap(long, default_value = "huus=info,warn")]
	tracing: String,
}

#[derive(Debug, Parser)]
enum SubCommand {
	Setup(setup::Setup),
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	tracing_subscriber::fmt()
		.with_env_filter(args.tracing)
		.init();

	match args.subcmd {
		SubCommand::Setup(setup) => {
			setup::setup(setup).await;
		}
	}
}
