use clap::{arg, Command};

use dotenv::from_filename;


pub fn read_cli(){
    let app = Command::new("rust-app")
    .arg(arg!("environment")
            .short('e')
            .long("env")
            .value_name("ENV_NAME")
            .help("Load env file")
            .value_parser(clap::builder::BoolValueParser::new())
    )
    .get_matches();

    if let Some(env) = app.get_one::<String>("environment") {
        if env == "development" || env == "dev" {
            from_filename(".env.development").expect("load env error");
        }
        else if env == "production" || env == "prod" { 
            from_filename(".env.production").expect("load env error");
        }
    }
}