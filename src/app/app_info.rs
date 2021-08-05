use super::{Company, InterviewInfo, Status};
use crate::cli::Jam;
use chrono::Local;

#[derive(Debug)]
pub struct AppInfo {
    company_info: Company,
    interview_info: InterviewInfo,
    status: Vec<Status>,
}

#[derive(Default, Debug)]
pub struct AppInfoBuilder {
    status: Vec<Status>,
    company_info: Company,
    interview_info: InterviewInfo,
}

impl AppInfoBuilder {
    pub fn new(args: Jam) -> Self {
        let mut app_info: AppInfoBuilder = Default::default();

        match args {
            Jam::Add {
                company,
                locations,
                date,
                interview_date,
                interviewer,
                interviewer_email,
                interviewer_tel,
            } => {
                app_info.set_status(date);
                app_info.set_company_info(
                    company.to_ascii_uppercase(),
                    locations
                        .iter()
                        .map(|loc| loc.to_ascii_uppercase())
                        .collect(),
                );
                app_info.set_interview_info(
                    interview_date,
                    interviewer,
                    interviewer_email,
                    interviewer_tel,
                );
            }
            _ => (),
        }

        app_info
    }

    fn set_status(&mut self, date: String) {
        if date.is_empty() {
            self.status.push(Status::Applied(Local::now().to_string()));
        } else {
            self.status.push(Status::Applied(date));
        }
    }

    fn set_company_info(&mut self, company: String, locations: Vec<String>) {
        let company_info = Company { company, locations };
        self.company_info = company_info;
    }

    fn set_interview_info(
        &mut self,
        interview_date: Option<String>,
        interviewer: Option<String>,
        interviewer_email: Option<String>,
        interviewer_tel: Option<String>,
    ) {
        let interview_info = InterviewInfo {
            interview_date,
            interviewer,
            interviewer_email,
            interviewer_tel,
        };
        self.interview_info = interview_info;
    }

    pub fn build(self) -> AppInfo {
        AppInfo {
            status: self.status,
            company_info: self.company_info,
            interview_info: self.interview_info,
        }
    }
}
