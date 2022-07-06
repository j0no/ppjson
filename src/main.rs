use clap::{value_parser, arg, command};
use serde_json::{from_str, to_string_pretty, Value, };
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;

static DEFAULT_PACKAGE_JSON_NAME: &str = "package.json";
static DEFAULT_KEY: &str = "scripts";

fn main() {
    let mut cmd = command!()
        .arg(arg!([INPUT_JSON_FILE] "a json file").value_parser(value_parser!(PathBuf)))
        .arg(arg!([KEY] "key").value_parser(value_parser!(String)))
        .arg(arg!(table: -t "format as table").value_parser(value_parser!(bool)));
    let matches = cmd.get_matches_mut();
    
    // TODO: Load params from env DEFAULT_JSON_FILE_NAME, DEFAULT_OBJECT_KEY

    // TODO: Read from params from command to get flattened keys or style   

    let input = matches
    .get_one::<PathBuf>("INPUT_JSON_FILE");
   
    let path_to_file : Option<PathBuf>= match input {
        Some(file_path) => {
            if file_path.is_file() {
               Some(file_path.to_owned())
            } else {
                let mut pwd = current_dir().unwrap();
                pwd.push(file_path);
                Some(pwd)
            }
        }  
        None => {
            let mut pwd = current_dir().unwrap();
            pwd.push(DEFAULT_PACKAGE_JSON_NAME);
            Some(pwd)
        }
    };

    let key_str = matches
    .get_one::<String>("KEY");

    let key_name = match key_str {
        Some(key) => key,
        None => DEFAULT_KEY
    };

    let format_as_tables = matches.contains_id("table"); 

    // get json file content
    let contents = read_to_string(path_to_file.unwrap()).expect("Something went wrong reading the file");
    let json: Value = from_str(&contents).expect("JSON was not well-formatted");
    let json_string = json[key_name].to_string();

    // make new json object
    let formatted_json: String = format!("{{ \"{}\": {} }}", key_name, json_string);
    let formatted_json_parsed: Value =
        from_str(&formatted_json).expect("JSON was not well-formatted");


    if format_as_tables {
        print!("print as table");
    } else {
        // print package.json scripts
        let pretty_str = to_string_pretty(&formatted_json_parsed).expect("failed");
        // println!("{}", DEFAULT_PACKAGE_JSON_NAME);
        print!("{}", pretty_str);   
    }
    

}
