use headless_chrome::{Browser, Tab};
use std::{sync::Arc, thread, time::Duration};

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

        self.tab
            .wait_for_element_with_custom_timeout(
                "input#employee_session_form_office_account_name",
                Duration::from_secs(60),
            )?
            .click()?;
        self.tab.type_str(&self.login_info.company)?;

        self.tab
            .wait_for_element_with_custom_timeout(
                "input#employee_session_form_account_name_or_email",
                Duration::from_secs(60),
            )?
            .click()?;
        self.tab.type_str(&self.login_info.account)?;

        self.tab
            .wait_for_element_with_custom_timeout(
                "input#employee_session_form_password",
                Duration::from_secs(60),
            )?
            .click()?;
        self.tab.type_str(&self.login_info.password)?;

        self.tab
            .wait_for_element_with_custom_timeout(
                "input.attendance-button-email",
                Duration::from_secs(60),
            )?
            .click()?;

        self.tab.wait_for_element_with_custom_timeout(
            "h1.attendance-category-title",
            Duration::from_secs(5),
        )?;

        thread::sleep(Duration::from_secs(2));

        Ok(())
    }

    pub fn attendance(&self) -> Result<String, failure::Error> {
        self.tab
            .wait_for_element_with_custom_timeout(
                "#kt-attendance-card-time-stamp > ul > li:nth-child(1) > form",
                Duration::from_secs(60),
            )?
            .click()?;

        let date = self.get_date()?;
        let time = self.get_time()?;
        Ok(format!("{} {}", date, time))
    }

    pub fn leaving(&self) -> Result<String, failure::Error> {
        self.tab
            .wait_for_element_with_custom_timeout(
                "#kt-attendance-card-time-stamp > ul > li:nth-child(2) > form",
                Duration::from_secs(60),
            )?
            .click()?;

        let date = self.get_date()?;
        let time = self.get_time()?;
        Ok(format!("{} {}", date, time))
    }

    fn get_date(&self) -> Result<String, failure::Error> {
        let date = self
            .tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-date",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();

        Ok(date)
    }

    fn get_time(&self) -> Result<String, failure::Error> {
        let time = self
            .tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-time",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();

        Ok(time)
    }
}
