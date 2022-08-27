use clap::Parser;

mod net;

/// Binary application that runs a whyrc server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    ip_address: String,

    #[clap(short = 'P', long, value_parser, default_value = "7341")]
    port: u16,

    #[clap(short, long, value_parser)]
    password: Option<String>, // if unspecified, the server will generate one
}

fn main() {
    let args = Args::parse();

    net::start_server(args);
}
