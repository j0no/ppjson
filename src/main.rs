use clap::{arg, command, value_parser};
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use serde_json::Map;
use serde_json::Value::{Array, Bool, Null, Number, Object, String as JSONString};
use serde_json::{from_str, to_string_pretty, Value};
use std::env;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::Split;
use tabled::{Style, Table, Tabled};

#[derive(Tabled, Debug)]
struct JsonTable {
    key: String,
    value: String,
    value_type: String,
}

fn main() {
    let mut cmd = command!()
        .arg(arg!([INPUT_JSON_FILE] "a json file").value_parser(value_parser!(PathBuf)))
        .arg(
            arg!([KEY] "key")
                .value_parser(value_parser!(String))
                .required(false),
        )
        .arg(arg!(table: -t "format as table").value_parser(value_parser!(bool)))
        .arg(arg!(print_keys: -k "print keys").value_parser(value_parser!(bool)))
        .arg(
            arg!(key_delimiter: -d <DELIMETER>"set key delimeter")
                .value_parser(value_parser!(String)),
        );

    let matches = cmd.get_matches_mut();

    // TODO: Read from params from command to get flattened keys or style

    let input = matches.get_one::<PathBuf>("INPUT_JSON_FILE");

    let path_to_file: Option<PathBuf> = match input {
        Some(file_path) => {
            if file_path.is_file() {
                Some(file_path.to_owned())
            } else {
                let mut pwd = current_dir().unwrap();
                pwd.push(file_path);
                Some(pwd)
            }
        }
        None => None,
    };

    let key_str = matches.get_one::<String>("KEY");

    let key_delimiter = matches.get_one::<String>("key_delimiter");

    let keys_arr = match key_str {
        Some(key_str_val) => match key_delimiter {
            Some(key_delimiter_val) => Some(key_str_val.split(key_delimiter_val)),
            None => None,
        },
        None => None,
    };

    let print_keys = matches.contains_id("print_keys");
    let format_as_tables = matches.contains_id("table");

    // get json file content
    let contents =
        read_to_string(path_to_file.expect("File not specified")).expect("File doesn't exists");
    let json: Value = from_str(&contents).expect("JSON was not well-formatted");

    if format_as_tables {
        match keys_arr {
            Some(keys_arr_val) => {
                let mut init = false;
                let mut past_val: &Value = &Null;
                for val in keys_arr_val.into_iter() {
                    if !init {
                        past_val = &json[val];
                        init = true;
                    } else {
                        past_val = &past_val[val];
                    }
                }

                let rows = get_rows(past_val);

                let mut table = Table::new(rows);
                table.with(Style::modern());
                println!("{}", table.to_string())
            }
            None => {
                let rows = match key_str {
                    Some(key_name) => get_rows(&json[key_name]),
                    None => get_rows(&(json)),
                };
                let mut table = Table::new(rows);
                table.with(Style::modern());
                println!("{}", table.to_string())
            }
        };
    } else {
        if print_keys {
            match key_str {
                Some(key_name) => {
                    let flattened = if key_name.eq(&".".to_string()) {
                        Flattener::new()
                            .set_key_separator(".")
                            .set_array_formatting(ArrayFormatting::Surrounded {
                                start: "[".to_string(),
                                end: "]".to_string(),
                            })
                            .set_preserve_empty_arrays(false)
                            .set_preserve_empty_objects(false)
                            .flatten(&json)
                            .unwrap()
                    } else {
                        Flattener::new()
                            .set_key_separator(".")
                            .set_array_formatting(ArrayFormatting::Surrounded {
                                start: "[".to_string(),
                                end: "]".to_string(),
                            })
                            .set_preserve_empty_arrays(false)
                            .set_preserve_empty_objects(false)
                            .flatten(&json[key_name])
                            .unwrap()
                    };

                    match flattened {
                        Object(flt_obj) => {
                            let obj_keys = flt_obj.keys().collect::<Vec<_>>();
                            for key_name in obj_keys {
                                println!("\t{}", key_name);
                            }
                        }
                        Null => {
                            println!("No keys");
                        }
                        Bool(_) => {
                            println!("No keys");
                        }
                        Number(_) => {
                            println!("No keys");
                        }
                        Array(_) => {
                            println!("No keys");
                        }
                        JSONString(_) => {
                            println!("No keys");
                        }
                    }
                }
                None => {
                    let flattened = Flattener::new()
                        .set_key_separator(".")
                        .set_array_formatting(ArrayFormatting::Surrounded {
                            start: "[".to_string(),
                            end: "]".to_string(),
                        })
                        .set_preserve_empty_arrays(false)
                        .set_preserve_empty_objects(false)
                        .flatten(&json)
                        .unwrap();

                    match flattened {
                        Object(flt_obj) => {
                            let obj_keys = flt_obj.keys().collect::<Vec<_>>();
                            for key_name in obj_keys {
                                println!("\t{}", key_name);
                            }
                        }
                        Null => {
                            println!("No keys");
                        }
                        Bool(_) => {
                            println!("No keys");
                        }
                        Number(_) => {
                            println!("No keys");
                        }
                        Array(_) => {
                            println!("No keys");
                        }
                        JSONString(_) => {
                            println!("No keys");
                        }
                    }
                }
            };
        } else {
            match keys_arr {
                Some(keys_arr_val) => {
                    let keys_arr_vec: Vec<&str> = keys_arr_val.collect();
                    let mut init = false;
                    let mut past_val: &Value = &Null;
                    for val in keys_arr_vec.clone().into_iter() {
                        if !init {
                            past_val = &json[val];
                            init = true;
                        } else {
                            past_val = &past_val[val];
                        }
                    }
                    
                    let mut inserted_main_content = false; 
                    
                    let mut final_map: Map<String, Value> = Map::new();
                    for val in keys_arr_vec.into_iter().rev() {
                        if inserted_main_content {
                            let mut wrap_map = Map::new();
                            wrap_map.insert(val.to_string(), final_map.clone().into());
                            final_map = wrap_map;
                        } else {
                            let mut main_map = Map::new();
                            main_map.insert(val.to_string(), past_val.clone());
                            final_map = main_map;
                            inserted_main_content = true;
                        }
                        
                    }

                    let custom_json: Value = final_map.into();
                    let pretty_str = to_string_pretty(&custom_json).expect("failed");
                    // make new json object
                    print!("{}", pretty_str);
                }
                None => {
                    print!("{}", to_string_pretty(&json).expect("failed"));
                }
            };
        }
    }
}
fn _get_json_value(json: &Value, keys_arr_val: Split<&str>) -> Value {
    let mut init = false;
    let mut past_val: &Value = &Null;
    for val in keys_arr_val.into_iter() {
        if !init {
            past_val = &json[val];
            init = true;
        } else {
            past_val = &past_val[val];
        }
    }
    past_val.clone()
}
fn get_rows(json: &Value) -> Vec<JsonTable> {
    let mut rows: Vec<JsonTable> = Vec::new();

    match &json {
        Object(flt_obj) => {
            // let obj_keys = flt_obj.keys().collect::<Vec<_>>();
            for (key, value) in flt_obj.into_iter() {
                let value_type = match &value {
                    Object(_flt_obj) => "Object",
                    Null => "Null",
                    Bool(_) => "Bool",
                    Number(_) => "Number",
                    Array(_) => "Array",
                    JSONString(_) => "String",
                };

                let pretty_str = to_string_pretty(&value).expect("failed");
                rows.push(JsonTable {
                    key: key.to_string(),
                    value: pretty_str.to_string(),
                    value_type: value_type.to_string(),
                });
            }
        }
        Null => {
            println!("No keys");
        }
        Bool(_) => {
            println!("No keys");
        }
        Number(_) => {
            println!("No keys");
        }
        Array(_) => {
            println!("No keys");
        }
        JSONString(_) => {
            println!("No keys");
        }
    }
    rows
}
