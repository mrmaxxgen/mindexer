// mindexer (c) mrmaxxgen 2026
// licensed under GNU GPLv3 or later <https://www.gnu.org/licenses/>

use std::io;
use std::fs;
use std::env;
use glob::glob;
use std::path::Path;

fn main() {    
    // mindexer's purpose is very simple: to sort all files in one big folder called "sortfolder" into multiple smaller folder
    // for file type like "videos", "songs", "repos" ecc... it works by spawning shells commands that moves each file into a
    // specific folder by it's type, like "song.mp3" from "sortfolder" directory to "songs" directory.
    // if mindexer doesn't detects the base, it will create it, or, if it's not running inside the base, it will simply
    // outputs an error message and quit. throughout the base making and file sorting, each file and base modification will
    // be logged and reported on the terminal like "[ok] sorting file <FILE> into folder <FOLDER>". that's all! :3
    // for compilation and usage instructions, see README.MD or mindexer github page <https://github.com/mrmaxxgen/mindexer.git>

    // steps: 
    //        - check if mindexerbase exists.
    //        - cd into mindexerbase.
    //        ! move videos from "sortfolder" dir to "videos" dir.
    //        ! move songs from "sortfolder" dir to "songs" dir.

    // structure: 
    //            - core function (files indexing)
    //            - base creator

    // issues: 
    //         TODO: allow wildcards with glob
    //         TODO: solve loop-printing in base creator section

    if Path::new("mindexerbase").is_dir() { // check if mindexerbase directory exists
        match env::set_current_dir("mindexerbase") { // cd into mindexerbase directory
            Ok(()) => {
                match fs::copy("sortfolder/*.mp4", "videos/*.mp4") { // ! moves videos to videos directory
                    Ok(_) => {
                        println!("[i] videos moved succesfully.") // prints info if it moved videos succesfully
                    }
                    Err(e) => {
                        println!("[!] unable to move videos, {}", e) // prints error if it didn't moved videos
                    }
                }
                match fs::copy("sortfolder/*.mp3", "songs/*.mp3") { // ! moves songs to songs directory
                    Ok(_) => {
                        println!("[i] songs moved succesfully.") // prints info if it moved songs succesfully
                    }
                    Err(e) => {
                        println!("[!] unable to move songs, {}", e) // prints error if it didn't moved songs
                    }
                }
            }  
            Err(e) => {
                println!("[!] unable to cd into mindexerbase, {}", e) // prints error if it didn't cd into mindexerbase
            }
        }

    } else {
        println!("[?] mindexerbase not found, create it? (y/n):"); // asks user if he wants to create mindexerbase
        let mut base_maker_ask = String::new();
        io::stdin().read_line(&mut base_maker_ask).expect("failed to read line");
        if base_maker_ask.trim() == "y" {
            let basedirs = ["mindexerbase/sortfolder", "mindexerbase/videos", "mindexerbase/songs"];
            for dir in basedirs {
                match fs::create_dir_all(dir) { // creates mindexerbase if user agreed
                    Ok(_) => {
                        println!("[i] mindexerbase created succesfully, rerun.") // prints info if it created mindexerbase succesfully
                    }
                    Err(e) => {
                        println!("[!] unable to create mindexerbase, {}", e) // prints error if it didn't created mindexerbase
                    }
                }
            }
        } else {
            println!("[i] aborting") // do nothing if user declined
        }
    }
}