mod lib;

use headless_chrome::Browser;
use lib::Dakoku;
use seahorse::{App, Command, Context, Flag, FlagType};
use spinners::{Spinner, Spinners};
use std::{env, process};

fn main() {
    let args = env::args().collect();
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage(format!("{} [command] [flags]", env!("CARGO_PKG_NAME")))
        .version(env!("CARGO_PKG_VERSION"))
        .command(down_command())
        .command(up_command())
        .run(args);
}

fn down_command() -> Command {
    Command::new("down")
        .alias("d")
        .description("Reporting for work")
        .usage(format!("{} down [flags]", env!("CARGO_PKG_NAME")))
        .flag(company_flag())
        .flag(account_flag())
        .flag(password_flag())
        .action(|c| {
            let sp = Spinner::new(&Spinners::Moon, "Waiting...".into());
            let company = get_company(c);
            let account = get_account(c);
            let password = get_password(c);

            let dakoku = Dakoku::new(company, account, password);
            let browser = Browser::default().unwrap();
            let tab = browser.wait_for_initial_tab().unwrap();

            match dakoku.login(&tab) {
                Ok(_) => match dakoku.down(&tab) {
                    Ok(s) => println!("\rSuccess: {}", s),
                    Err(_) => println!("\rError..."),
                },
                Err(_) => eprintln!("\rError..."),
            }

            sp.stop()
        })
}

fn up_command() -> Command {
    Command::new("up")
        .alias("u")
        .description("Leaving work")
        .usage(format!("{} up [flags]", env!("CARGO_PKG_NAME")))
        .flag(company_flag())
        .flag(account_flag())
        .flag(password_flag())
        .action(|c| {
            let sp = Spinner::new(&Spinners::Moon, "Waiting...".into());
            let company = get_company(c);
            let account = get_account(c);
            let password = get_password(c);

            let dakoku = Dakoku::new(company, account, password);
            let browser = Browser::default().unwrap();
            let tab = browser.wait_for_initial_tab().unwrap();

            match dakoku.login(&tab) {
                Ok(_) => match dakoku.up(&tab) {
                    Ok(s) => println!("\rSuccess: {}", s),
                    Err(_) => println!("\rError..."),
                },
                Err(_) => eprintln!("\rError..."),
            }

            sp.stop()
        })
}

fn company_flag() -> Flag {
    Flag::new("company", FlagType::String)
        .alias("c")
        .description("Company ID")
}

fn account_flag() -> Flag {
    Flag::new("account", FlagType::String)
        .alias("a")
        .description("Account ID or email address")
}

fn password_flag() -> Flag {
    Flag::new("password", FlagType::String)
        .alias("p")
        .alias("pass")
        .description("Password")
}

fn get_company(c: &Context) -> String {
    match c.string_flag("company") {
        Ok(company) => company,
        Err(_) => match env::var("DAKOKU_COMPANY") {
            Ok(company) => company,
            Err(_) => {
                eprintln!("\rNot found enviroment variable \"DAKOKU_COMPANY\" nor command line argument \"--company.\"");
                process::exit(1);
            }
        },
    }
}

fn get_account(c: &Context) -> String {
    match c.string_flag("account") {
        Ok(account) => account,
        Err(_) => match env::var("DAKOKU_ACCOUNT") {
            Ok(account) => account,
            Err(_) => {
                eprintln!("\rNot found enviroment variable \"DAKOKU_ACCOUNT\" nor command line argument \"--account\".");
                process::exit(1);
            }
        },
    }
}

fn get_password(c: &Context) -> String {
    match c.string_flag("password") {
        Ok(pass) => pass,
        Err(_) => match env::var("DAKOKU_PASSWORD") {
            Ok(pass) => pass,
            Err(_) => {
                eprintln!("\rNot found enviroment variable \"DAKOKU_PASSWORD\" nor command line argument \"--password\".");
                process::exit(1);
            }
        },
    }
}
