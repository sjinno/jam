fn main() {
    // Create database connection
    // Alert if database is not found

    // Get and parse arguments
    for argument in std::env::args().skip(1) {
        println!("{}", argument);
    }

    // If add, create an item with fields:
    // company_name(input required)
    // date_applied(default: current time)
    // location(input required)
    // status(default: applied)
    // And add it to the database
}

// Available commands:
// add
// show <id> | <company name> | <date> | <location>
// edit <id>
// delete <id>
// head(default: the latest 10 companies) | [-n <n> where n is the latest n companies applied]
// count --- shows total number of jobs applied
