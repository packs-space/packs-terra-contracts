use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::remove_schemas;

// use packs::dao::ConfigResponse;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // export_schema(&schema_for!(ConfigResponse), &out_dir);
}
