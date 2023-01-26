use clap::{value_parser, arg, command};
use serde_json::{from_str, to_string_pretty, Value };
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use serde_json::Value::{Object, Null, Bool, Number, Array, String as JSONString};
use std::env;
use tabled::{Tabled};



#[derive(Tabled, Debug)]
struct JsonTable {
    key: String,
    value: String
}

fn main() {
    let mut cmd = command!()
        .arg(arg!([INPUT_JSON_FILE] "a json file").value_parser(value_parser!(PathBuf)))
        .arg(arg!([KEY] "key").value_parser(value_parser!(String)).required(false))
        .arg(arg!(table: -t "format as table").value_parser(value_parser!(bool)))
        .arg(arg!(print_keys: -k "print keys").value_parser(value_parser!(bool)));
    let matches = cmd.get_matches_mut();
    
    // TODO: Load params from env DEFAULT_JSON_FILE_NAME, DEFAULT_OBJECT_KEY

    // TODO: Read from params from command to get flattened keys or style   

    let input = matches
    .get_one::<PathBuf>("INPUT_JSON_FILE");
   
    let path_to_file : Option<PathBuf> = match input {
        Some(file_path) => {
            if file_path.is_file() {
               Some(file_path.to_owned())
            } else {
                let mut pwd = current_dir().unwrap();
                pwd.push(file_path);
                Some(pwd)
            }
        }  
        None => None
    };

    let key_str = matches
    .get_one::<String>("KEY");

    let print_keys = matches.contains_id("print_keys"); 
    let format_as_tables = matches.contains_id("table");

    // get json file content
    let contents = read_to_string(path_to_file.expect("File not specified")).expect("File doesn't exists");
    let json: Value = from_str(&contents).expect("JSON was not well-formatted");


    if format_as_tables {
        println!("print as table");
        // let mut rows : Vec<JsonTable> = Vec::new();
    
        // match &json {
        //     Object(flt_obj) =>{ 
        //         // let obj_keys = flt_obj.keys().collect::<Vec<_>>();
        //         for (key, value) in flt_obj.into_iter() {
        //             let pretty_str = to_string_pretty(&value).expect("failed");
        //             rows.push(JsonTable { key: key.to_string(), value: pretty_str.to_string() });
        //         }
        //      },
        //     Null =>{ println!("No keys"); },
        //     Bool(_) =>{ println!("No keys"); },
        //     Number(_) =>{ println!("No keys"); },
        //     Array(_) =>{ println!("No keys"); },
        //     JSONString(_) =>{ println!("No keys"); }
        // }

        // let table = Table::new(rows).to_string();
        // println!("{}", table)
        
    } else {
        if print_keys {            
            match key_str {
                Some(key_name) => {
                    let flattened =  if key_name.eq(&".".to_string()) {
                        Flattener::new()
                        .set_key_separator(".")
                        .set_array_formatting(ArrayFormatting::Surrounded {
                            start: "[".to_string(),
                            end: "]".to_string()
                        })
                        .set_preserve_empty_arrays(false)
                        .set_preserve_empty_objects(false)
                        .flatten(&json).unwrap()
                    } else {
                        Flattener::new()
                        .set_key_separator(".")
                        .set_array_formatting(ArrayFormatting::Surrounded {
                            start: "[".to_string(),
                            end: "]".to_string()
                        })
                        .set_preserve_empty_arrays(false)
                        .set_preserve_empty_objects(false)
                        .flatten(&json[key_name]).unwrap()
                    };
                   

                    match flattened {
                        Object(flt_obj) =>{ 
                            let obj_keys = flt_obj.keys().collect::<Vec<_>>();
                            for key_name in obj_keys {
                                println!("\t{}", key_name);
                            }
                         },
                        Null =>{ println!("No keys"); },
                        Bool(_) =>{ println!("No keys"); },
                        Number(_) =>{ println!("No keys"); },
                        Array(_) =>{ println!("No keys"); },
                        JSONString(_) =>{ println!("No keys"); }
                    }
                },
                None => {
                    let flattened = Flattener::new()
                        .set_key_separator(".")
                        .set_array_formatting(ArrayFormatting::Surrounded {
                            start: "[".to_string(),
                            end: "]".to_string()
                        })
                        .set_preserve_empty_arrays(false)
                        .set_preserve_empty_objects(false)
                        .flatten(&json).unwrap();
  
                    match flattened {
                        Object(flt_obj) =>{ 
                            let obj_keys = flt_obj.keys().collect::<Vec<_>>();
                            for key_name in obj_keys {
                                println!("\t{}", key_name);
                            }
                         },
                        Null =>{ println!("No keys"); },
                        Bool(_) =>{ println!("No keys"); },
                        Number(_) =>{ println!("No keys"); },
                        Array(_) =>{ println!("No keys"); },
                        JSONString(_) =>{ println!("No keys"); }
                    }
                }
            };
        } else {
            match key_str {
                Some(key_name) => {
                    let formatted_json: String = format!("{{ \"{}\": {} }}", key_name, json[key_name].to_string());
                    let formatted_json_parsed: Value =
                        from_str(&formatted_json).expect("JSON was not well-formatted");
                    let pretty_str = to_string_pretty(&formatted_json_parsed).expect("failed");
                    // make new json object
                    print!("{}", pretty_str);   
                },
                None => {
                    print!("{}", to_string_pretty(&json).expect("failed"));
                }
            };

        }
    }
    

}
