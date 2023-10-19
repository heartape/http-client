use std::env;
use http_client::cli;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = cli::parse(args[1..].to_vec());
    cli::cli_to_request(cli).do_http();
}

