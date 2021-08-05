use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq, Eq)]
#[structopt(about = "the stupid content tracker")]
pub enum Jam {
    /// Add a job you have applied to.
    Add {
        /// Company name.
        #[structopt(short = "c", long = "company")]
        company: String,
        /// Company location/ Your work location. E.g. Portland Remote.
        #[structopt(short = "l", long = "location")]
        locations: Vec<String>,
        //# Default data:
        /// If not specified, the current local date and time will be assigned.
        #[structopt(short = "d", long = "date", default_value = "")]
        date: String,
        //# Default data ends.

        //# Optional data:
        //### Interview info:
        /// Interview date.
        #[structopt(short = "i", long = "interview-date")]
        interview_date: Option<String>,
        /// Interviewer's name.
        #[structopt(short = "n", long = "interviewer-name")]
        interviewer_name: Option<String>,
        /// Interviewr's email.
        #[structopt(short = "e", long = "interviewer-email")]
        interviewer_email: Option<String>,
        /// Interviewr's phone number.
        #[structopt(short = "t", long = "interviewer-tel")]
        interviewer_tel: Option<String>,
        //### Interview info ends.
        //# Optional data ends.
    },
    Fetch {
        #[structopt(long)]
        dry_run: bool,
        #[structopt(long)]
        all: bool,
        repository: Option<String>,
    },
    Commit {
        #[structopt(short)]
        message: Option<String>,
        #[structopt(short)]
        all: bool,
    },
}

impl Jam {
    pub fn new() -> Self {
        Jam::from_args()
    }
}
