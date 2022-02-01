use headless_chrome::Tab;
use std::{sync::Arc, thread, time::Duration};

pub struct Dakoku {
    company: String,
    account: String,
    password: String,
}

impl Dakoku {
    pub fn new(company: String, account: String, password: String) -> Self {
        Self {
            company,
            account,
            password,
        }
    }

    pub fn login(&self, tab: &Arc<Tab>) -> Result<(), failure::Error> {
        let url = "https://attendance.moneyforward.com/employee_session/new";
        tab.navigate_to(url)?;

        tab.wait_for_element_with_custom_timeout(
            "input#employee_session_form_office_account_name",
            Duration::from_secs(60),
        )?
        .click()?;
        tab.type_str(&self.company)?;

        tab.wait_for_element_with_custom_timeout(
            "input#employee_session_form_account_name_or_email",
            Duration::from_secs(60),
        )?
        .click()?;
        tab.type_str(&self.account)?;

        tab.wait_for_element_with_custom_timeout(
            "input#employee_session_form_password",
            Duration::from_secs(60),
        )?
        .click()?;
        tab.type_str(&self.password)?;

        tab.wait_for_element_with_custom_timeout(
            "input.attendance-button-email",
            Duration::from_secs(60),
        )?
        .click()?;

        tab.wait_for_element_with_custom_timeout(
            "h1.attendance-category-title",
            Duration::from_secs(5),
        )?;

        thread::sleep(Duration::from_secs(2));

        Ok(())
    }

    pub fn down(&self, tab: &Arc<Tab>) -> Result<String, failure::Error> {
        tab.wait_for_element_with_custom_timeout(
            "#kt-attendance-card-time-stamp > ul > li:nth-child(1) > form",
            Duration::from_secs(60),
        )?
        .click()?;

        let date = tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-date",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        let time = tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-time",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        Ok(format!("{} {}", date, time))
    }

    pub fn up(&self, tab: &Arc<Tab>) -> Result<String, failure::Error> {
        tab.wait_for_element_with_custom_timeout(
            "#kt-attendance-card-time-stamp > ul > li:nth-child(2) > form",
            Duration::from_secs(60),
        )?
        .click()?;

        let date = tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-date",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        let time = tab
            .wait_for_element_with_custom_timeout(
                "div.attendance-card-time-recorder-time",
                Duration::from_secs(60),
            )?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        Ok(format!("{} {}", date, time))
    }
}
