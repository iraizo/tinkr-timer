use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version)]
pub struct Configuration {
    // tinkr executeables which should be ran
    #[arg(
        long,
        env,
        required = true,
        use_value_delimiter = true,
        value_delimiter = ' '
    )]
    pub executeables: Vec<String>,
    // amount in minutes
    #[arg(short, env, default_value = "10")]
    pub short_break: u16,
    // amount in minutes
    #[arg(short, env, default_value = "90")]
    pub long_break: u16,
}

pub fn parse() -> Configuration {
    Configuration::parse()
}
