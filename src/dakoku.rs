use chrono::Local;
use headless_chrome::{Browser, Element, Tab};
use std::{sync::Arc, time::Duration};

pub struct LoginInfo {
    company: String,
    account: String,
    password: String,
}

impl LoginInfo {
    pub fn new(company: String, account: String, password: String) -> Self {
        Self {
            company,
            account,
            password,
        }
    }
}

pub struct Dakoku {
    login_info: LoginInfo,
    #[allow(dead_code)]
    browser: Browser,
    tab: Arc<Tab>,
}

impl Dakoku {
    pub fn new(company: String, account: String, password: String) -> Self {
        let login_info = LoginInfo::new(company, account, password);
        let browser = Browser::default().unwrap();
        let tab = browser.new_tab().unwrap();
        Self {
            login_info,
            browser,
            tab,
        }
    }

    pub fn login(&self) -> Result<(), anyhow::Error> {
        let url = "https://attendance.moneyforward.com/employee_session/new";
        self.tab.navigate_to(url)?;

        self.click("input#employee_session_form_office_account_name")?;
        self.tab.type_str(&self.login_info.company)?;

        self.click("input#employee_session_form_account_name_or_email")?;
        self.tab.type_str(&self.login_info.account)?;

        self.click("input#employee_session_form_password")?;
        self.tab.type_str(&self.login_info.password)?;

        self.click("body > div.attendance-contents > div > div > div > div > form > div.attendance-before-login-card-button > input")?;

        self.select_element("h1.attendance-category-title")?;

        Ok(())
    }

    pub fn attendance(&self) -> Result<String, anyhow::Error> {
        self.click("div.clock_in")?;
        Ok(Local::now()
            .format("%Y年%m月%d日 %H時%M分%S秒 %Z")
            .to_string())
    }

    pub fn leaving(&self) -> Result<String, anyhow::Error> {
        self.click("div.clock_out")?;
        Ok(Local::now()
            .format("%Y年%m月%d日 %H時%M分%S秒 %Z")
            .to_string())
    }

    fn select_element(&self, selector: &str) -> Result<Element, anyhow::Error> {
        let element = self
            .tab
            .wait_for_element_with_custom_timeout(selector, Duration::from_secs(60))?;

        Ok(element)
    }

    fn click(&self, selector: &str) -> Result<(), anyhow::Error> {
        self.select_element(selector)?.click()?;

        Ok(())
    }
}
