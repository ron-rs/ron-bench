use ron::ser::to_writer;
use serde_json::from_reader as json_from_reader;
use std::fs::File;

// Convert .json to .ron
fn main() {
    let value: serde_json::Value =
        json_from_reader(File::open("../../data/canada.json").unwrap()).unwrap();
    to_writer(File::create("../../data/canada.ron").unwrap(), &value).unwrap();
}
