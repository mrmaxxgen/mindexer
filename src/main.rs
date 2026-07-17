// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

use std::io;
use std::fs;
use glob::glob;
use std::path::Path;

fn main() {    

    let basedirs_checker_list = ["mindexerbase", "mindexerbase/sortfolder", "mindexerbase/songs", "mindexerbase/videos"];

    for basedir in basedirs_checker_list {
        if Path::new(basedir).is_dir() {
        } else {
            match fs::create_dir(basedir) {
                Ok(_) => {
                    println!("[i] created missing directory: {}", basedir);
                }
                Err(e) => {
                    println!("[e] unable to crete missing directory {}: {}", basedir, e);
                }
            }
        }
    }
    
    
}