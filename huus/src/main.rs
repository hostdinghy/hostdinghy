/*!
This is the application which runs on the server itself.

It can be used as cli and also runs a gprc server.

How to install this
*/

mod apps;
mod docker;
mod postgresql;
mod registry;
#[cfg(debug_assertions)]
mod runtime_test;
mod server;
mod setup;
mod traefik;
mod utils;

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
	Registry(registry::Registry),
	Postgresql(postgresql::Postgresql),
	Serve,
	#[cfg(debug_assertions)]
	Test(runtime_test::Test),
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
		SubCommand::Registry(registry) => {
			registry::registry(registry).await;
		}
		SubCommand::Postgresql(postgresql) => {
			postgresql::postgresql(postgresql).await;
		}
		SubCommand::Serve => {
			server::serve().await;
		}
		#[cfg(debug_assertions)]
		SubCommand::Test(t) => {
			runtime_test::test(t).await;
		}
	}
}
