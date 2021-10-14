#[derive(Queryable)]
pub struct Application {
    pub id: i32,
    pub company: String,
    pub location: String,
    pub date: String,
    pub interview_date: String,
    pub status: String,
    pub hired: bool,
}
