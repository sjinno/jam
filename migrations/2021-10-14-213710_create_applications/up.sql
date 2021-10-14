-- Your SQL goes here
CREATE TABLE applications (
    id SERIAL PRIMARY KEY,
    company VARCHAR(30) NOT NULL,
    location VARCHAR(100) NOT NULL,
    date VARCHAR(30) NOT NULL,
    status VARCHAR(30) NOT NULL,
    interview_date VARCHAR(30),
    hired BOOLEAN NOT NULL DEFAULT 'f'
)