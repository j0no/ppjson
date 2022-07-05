use clap::App;
use serde_json::{from_str, to_string_pretty, Value};
use std::env::current_dir;
use std::fs::read_to_string;

static DEFAULT_PACKAGE_JSON_NAME: &str = "package.json";
static DEFAULT_KEY: &str = "scripts";

fn main() {
    let _matches = App::new("ppjson")
        .version("0.1.0")
        .author("j0no")
        .about("print json")
        .get_matches();

    // get pwd
    let mut pwd = current_dir().unwrap();
    // get package.json
    pwd.push(DEFAULT_PACKAGE_JSON_NAME);
    let contents = read_to_string(pwd).expect("Something went wrong reading the file");
    let json: Value = from_str(&contents).expect("JSON was not well-formatted");
    let json_string = json[DEFAULT_KEY].to_string();


    // make new json object
    let formatted_json: String = format!("{{ \"{}\": {} }}", DEFAULT_KEY, json_string);
    let formatted_json_parsed: Value =
        from_str(&formatted_json).expect("JSON was not well-formatted");

    // print package.json scripts
    let pretty_str = to_string_pretty(&formatted_json_parsed).expect("failed ");
    // println!("{}", DEFAULT_PACKAGE_JSON_NAME);
    print!("{}", pretty_str);

}
