mod db;

use db::store::KVStore;
use std::fs::OpenOptions;

fn main() {
    let path = "data.db";

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Unable to open data file");

    let mut store = KVStore::load_from_file(path).expect("Unable to load from file");

    store.insert("username".into(), "admin".into(), &mut file).unwrap();
    store.insert("password".into(), "secret".into(), &mut file).unwrap();

    if let Some(val) = store.get("username") {
        println!("username = {}", val);
    }

    println!("All records:");
    for record in store.all_records() {
        println!("{} = {}", record.key, record.value);
    }
}
