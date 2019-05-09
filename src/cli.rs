use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

pub fn matches<'a>() -> ArgMatches<'a> {
    let config_arg = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true);

    let token_arg = Arg::with_name("token")
        .short("t")
        .help("Bot token")
        .takes_value(true)
        .required(true);

    let echo_id_subcommand = SubCommand::with_name("echo_id")
        .about("Watch Telegram updates for obtain chat ids")
        .arg(token_arg);

    App::new("logram")
        .version(crate_version!())
        .author("Max Eliseev <ralvke@gmail.com>")
        .about("Pipe log updates to Telegram")
        .arg(config_arg)
        .subcommand(echo_id_subcommand)
        .get_matches()
}
