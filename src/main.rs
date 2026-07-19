// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

use std::fs;
use std::path::Path;

fn main() {    
    
    // CONFIG VARIABLES
    let config_path = "/etc/mindexer/config.txt";
    let log_path = "/etc/mindexer/logs.txt";
    let mut config_variables = vec![];
    let basedirs_checker_list = ["mindexerbase", "mindexerbase/sortfolder", "mindexerbase/songs", "mindexerbase/videos"];
    
    // DECLARATIONS
    println!("Mindexer 0.2.0 (c), mrmaxxgen 2026\nsee https://github.com/mrmaxxgen/mindexer.git for usage and config syntax\n[config info] - expected config path: {}\n[logs info] - expected logs path: {}", config_path, log_path);
    
    // CONFIG CHECKER
    match fs::read_to_string(config_path) {
        Ok(config) => {
            
            // CONFIG PARSER
            if config.is_empty() {
                println!("[config warning] - config file is empty, aborting");
            } else {
                for variable in config.lines() {
                    config_variables.push(variable);
                }
                
                // STATUS
                if config_variables.contains(&"status") {
                    println!("[config info] - enabled status");
                } else {
                    println!("[config info] - disabled status");
                }
                
                // LOG
                if config_variables.contains(&"log") {
                    println!("[config info] - enabled logging, this session is going to get logged");
                } else {
                    println!("[config info] - disabled logging");
                }
                
                // BASE CHECKER
                for basedir in basedirs_checker_list {
                    if Path::new(basedir).is_dir() {
                        println!("[base info] - base exists, proceeding");
                        // indexer
                    } else {
                        match fs::create_dir(basedir) {
                            Ok(_) => {
                                println!("[base info] - created missing directory: {}", basedir);
                            }
                            Err(error) => {
                                println!("[base error] - unable to create missing directory {}: {}", basedir, error);
                            }
                        }
                    }
                }
            }
        }
        Err(error) => {
            println!("[config error] - unable to parse config: {}, aborting", error);
        }
    }    
}