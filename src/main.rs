mod db;
mod crypt;
mod constants;
mod init;
mod args;

use std::process;
use clap::Parser;
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
            match db_parser {
                Err(e) => {
                    eprintln!("Unable to parse pwdstore, file may be missing or corrupt");
                    eprintln!("{}", e);
                    process::exit(1);
                },
                Ok(parser) => {
                    match domain {
                        None => {
                            let domain_list = parser.list_domains();
                            for d in domain_list {
                                println!("{}", d);
                            }
                        },
                        Some(d) => {
                            let account_list = parser.list_accounts(d.to_owned());
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
                }
            }
        },
        args::Commands::Add(args::AddArgs { domain, username }) => {
            let db_parser = db::dbParser::DBParser::new(pwdstore, false);
            match db_parser {
                Err(e) => {
                    eprintln!("Unable to parse pwdstore, file may be missing or corrupt");
                    eprintln!("{}", e);
                    process::exit(1);
                },
                Ok(mut parser) => {
                    let password = crypt::get_passphrase(true, false).unwrap();
                    match parser.add_password(domain.to_owned(), username.to_owned(), password) {
                        Ok(_) => {
                            println!("Added password successfully");
                        },
                        Err(e) => {
                            eprintln!("Unable to add password");
                            eprintln!("{}", e);
                            process::exit(1);
                        }
                    }
                }
            }
        },
        args::Commands::Show(args::AddArgs { domain, username }) => {
            let db_parser = db::dbParser::DBParser::new(pwdstore, false);
            match db_parser {
                Err(e) => {
                    eprintln!("Unable to parse pwdstore, file may be missing or corrupt");
                    eprintln!("{}", e);
                    process::exit(1);
                },
                Ok(parser) => {
                    let password = parser.get_password(domain.to_owned(), username.to_owned()); 
                    match password {
                        Err(e) => {
                            eprintln!("Unable to decrypt password");
                            eprintln!("{}", e);
                            process::exit(1);
                        },
                        Ok(pwd) => {
                            match pwd {
                                None => {
                                    eprintln!("No password found");
                                    process::exit(1);
                                },
                                Some(p) => {
                                    match String::from_utf8(p) {
                                        Err(e) => {
                                            eprintln!("Invalid utf8 sequence found in password");
                                            eprintln!("{}", e);
                                            process::exit(1);
                                        },
                                        Ok(s) => {
                                            println!("{}", s);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
