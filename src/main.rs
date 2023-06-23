mod db;
mod crypt;
mod constants;
mod init;
mod args;

use clap::Parser;
use std::path::PathBuf;
use dirs;


fn main() {
    let mut pwdstore = dirs::home_dir().unwrap();
    pwdstore.push(constants::LOCAL_DIR);
    pwdstore.push(constants::PWD_STORE);
    let cli = args::Cli::parse();
    match &cli.command {
        args::Commands::Init { key_size } => {
            let initializer = init::Initializer::new(constants::LOCAL_DIR.to_string(),
                                                     constants::KEY_STORE.to_string(),
                                                     constants::PWD_STORE.to_string(),
                                                     key_size.clone());
            initializer.initialize();
        },
        args::Commands::List { domain } => {
            let db_parser = db::dbParser::DBParser::new(pwdstore, false);
            match domain {
                None => {
                    let domain_list = db_parser.unwrap().list_domains();
                    for d in domain_list {
                        println!("{}", d);
                    }
                },
                Some(d) => {
                    let account_list = db_parser.unwrap().list_accounts(d.to_owned());
                    match account_list {
                        None => (),
                        Some(list) => {
                            for a in list {
                                println!("{}", a);
                            }
                        }
                    }
                }
            }
        },
        args::Commands::Add(args::AddArgs { domain, username }) => {
            let db_parser = db::dbParser::DBParser::new(pwdstore, false);
            let password = crypt::get_passphrase(true, false).unwrap();
            match db_parser.unwrap().add_password(domain.to_owned(), username.to_owned(), password) {
                Ok(_) => {
                    println!("added password successfully");
                },
                Err(e) => {
                    eprintln!("unable to add password");
                    eprintln!("{}", e);
                }
            }
        },
        args::Commands::Show(args::AddArgs { domain, username }) => {
            let db_parser = db::dbParser::DBParser::new(pwdstore, false).unwrap();
            let password = db_parser.get_password(domain.to_owned(), username.to_owned()).unwrap(); 
            println!("{}", String::from_utf8(password.unwrap()).unwrap());
        }
    }
}
