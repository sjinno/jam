mod app;
mod cli;
mod db;

use app::AppInfoBuilder;
use cli::Jam;

fn main() {
    let args = Jam::new();
    let app_info = AppInfoBuilder::new(args);

    println!("{:#?}", app_info);
}
