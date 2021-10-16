use std::fs::File;
use ron::ser::to_writer;
use ron::Value;
use serde_json::from_reader as json_from_reader;

const MIN_NUM_VALUE: usize = 10_000;

fn gen_value(num: usize) -> Value {
    Value::Bool(true)
}

// Convert .json to .ron
fn main() {
    let value: serde_json::Value = json_from_reader(File::open("data/canada.json").unwrap()).unwrap();
    to_writer(File::create("data/canada.ron").unwrap(), &value).unwrap();
}
