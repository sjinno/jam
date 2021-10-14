table! {
    applications (id) {
        id -> Int4,
        company -> Varchar,
        location -> Varchar,
        date -> Varchar,
        status -> Varchar,
        interview_date -> Nullable<Varchar>,
        hired -> Bool,
    }
}
