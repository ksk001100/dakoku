use chrono::{Datelike, Local};
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
        let tab = browser.wait_for_initial_tab().unwrap();
        Self {
            login_info,
            browser,
            tab,
        }
    }

    pub fn login(&self) -> Result<(), failure::Error> {
        let url = "https://attendance.moneyforward.com/employee_session/new";
        self.tab.navigate_to(url)?;

        self.click("input#employee_session_form_office_account_name")?;
        self.tab.type_str(&self.login_info.company)?;

        self.click("input#employee_session_form_account_name_or_email")?;
        self.tab.type_str(&self.login_info.account)?;

        self.click("input#employee_session_form_password")?;
        self.tab.type_str(&self.login_info.password)?;

        self.click("input.attendance-button-email")?;

        self.select_element("h1.attendance-category-title")?;

        Ok(())
    }

    pub fn attendance(&self) -> Result<String, failure::Error> {
        self.click("body > div.attendance-contents > header > nav > ul > li:nth-child(1) > a")?;
        self.click("#kt-attendance-card-time-stamp > ul > li:nth-child(1) > form")?;

        let date = self.get_date()?;
        let time = self.get_time()?;
        Ok(format!("{} {}", date, time))
    }

    pub fn leaving(&self) -> Result<String, failure::Error> {
        self.click("body > div.attendance-contents > header > nav > ul > li:nth-child(1) > a")?;
        self.click("#kt-attendance-card-time-stamp > ul > li:nth-child(2) > form")?;

        let date = self.get_date()?;
        let time = self.get_time()?;
        Ok(format!("{} {}", date, time))
    }

    pub fn is_holiday(&self) -> Result<bool, failure::Error> {
        self.click("#kt-attendance-header-navigation-item-attendances > a")?;
        let day = Local::now().day();

        let selector = &format!("body > div.attendance-contents > div.attendance-contents-inner > div > div > div > div > section > section > div.daily-attendances-table > div.att-pc.tw-p-16.tw-pt-0 > section > section > section > section > div:nth-child({}) > div:nth-child(5) > p", day);
        let pattern = self.get_element_value(selector)?; 

        Ok(pattern.contains("休日"))
    }

    fn get_date(&self) -> Result<String, failure::Error> {
        let date = self.get_element_value("div.attendance-card-time-recorder-date")?;

        Ok(date)
    }

    fn get_time(&self) -> Result<String, failure::Error> {
        let time = self.get_element_value("div.attendance-card-time-recorder-time")?;

        Ok(time)
    }

    fn get_element_value(&self, selector: &str) -> Result<String, failure::Error> {
        let value = self
            .select_element(selector)?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();

        Ok(value)
    }

    fn select_element(&self, selector: &str) -> Result<Element, failure::Error> {
        let element = self
            .tab
            .wait_for_element_with_custom_timeout(selector, Duration::from_secs(60))?;

        Ok(element)
    }

    fn click(&self, selector: &str) -> Result<(), failure::Error> {
        self.select_element(selector)?.click()?;

        Ok(())
    }
}
