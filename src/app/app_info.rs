use super::{Company, InterviewInfo, Status};

use crate::cli::Jam;

use chrono::Local;

pub struct AppInfo {
    company_info: Company,
    interview_info: InterviewInfo,
    status: Vec<Status>,
}

#[derive(Default)]
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
                interviewer_name,
                interviewer_email,
                interviewer_tel,
            } => {
                if date.is_empty() {
                    app_info
                        .status
                        .push(Status::Applied(Local::now().to_string()));
                }
                let company_info = Company {
                    name: company,
                    locations: locations,
                };
                let interview_info = InterviewInfo {
                    interview_date: interview_date,
                    interviewer: interviewer_name,
                    interviewer_email: interviewer_email,
                    interviewer_tel: interviewer_tel,
                };
                app_info.company_info = company_info;
                app_info.interview_info = interview_info;
            }
            _ => (),
        }

        app_info
    }
}
