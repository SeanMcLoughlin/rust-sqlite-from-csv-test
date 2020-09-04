use sqlite;
use std::process::Command;

fn main() {
    get_db_from_csv();
    print_db_contents();
}

fn get_db_from_csv() {
    Command::new("sqlite3").arg("test.db").arg(".mode csv").arg(".import test.csv test_table").output().expect("Failed to convert CSV to sqlite3 DB");
}

fn print_db_contents() {
    let connection = sqlite::open("test.db").unwrap();
    let query = "SELECT command FROM test_table WHERE command='RD'";
    connection
    .iterate(query, |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    }).unwrap();
}
