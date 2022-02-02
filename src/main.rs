mod lib;

use dotenv::dotenv;
use headless_chrome::Browser;
use lib::Dakoku;
use seahorse::{App, Command, Context, Flag, FlagType};
use spinners::{Spinner, Spinners};
use std::env;

fn main() {
    dotenv().ok();
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage(format!("{} [command] [flags]", env!("CARGO_PKG_NAME")))
        .version(env!("CARGO_PKG_VERSION"))
        .command(down_command())
        .command(up_command())
        .run(env::args().collect());
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
        Err(_) => env!("DAKOKU_COMPANY").to_string(),
    }
}

fn get_account(c: &Context) -> String {
    match c.string_flag("account") {
        Ok(account) => account,
        Err(_) => env!("DAKOKU_ACCOUNT").to_string(),
    }
}

fn get_password(c: &Context) -> String {
    match c.string_flag("password") {
        Ok(pass) => pass,
        Err(_) => env!("DAKOKU_PASSWORD").to_string(),
    }
}
