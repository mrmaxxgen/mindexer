// mindexer (c) mrmaxxgen 2026
// licensed under GNU GPLv3 or later <https://www.gnu.org/licenses/>

use std::io;
use std::fs;
use glob::glob;
use std::path::Path;

fn main() {    
    // mindexer's purpose is very simple: to sort all files in one big folder called "sortfolder" into multiple smaller folder
    // for file type like "videos", "songs", "repos" ecc... it works by spawning shells commands that moves each file into a
    // specific folder by it's type, like "song.mp3" from "sortfolder" directory to "songs" directory.
    // if mindexer doesn't detects the base, it will create it, or, if it's not running inside the base, it will simply
    // outputs an error message and quit. throughout the base making and file sorting, each file and base modification will
    // be logged and reported on the terminal like "[i] sorting file <FILE> into folder <FOLDER>". that's all! :3
    // for compilation and usage instructions, see README.MD or mindexer github page <https://github.com/mrmaxxgen/mindexer.git>

    // steps: 
    //        - check if mindexerbase exists, if not, ask user to create it.
    //        - copy videos from "sortfolder" dir to "videos" dir.
    //        - copy songs from "sortfolder" dir to "songs" dir.
    //        - delete all files in "sortfolder" dir.

    // structure: 
    //            - core function (files indexing)
    //            - base creator

    // issues: 
    //            - no issues for now

    if Path::new("mindexerbase").is_dir() { // check if mindexerbase directory exists
        for entry in glob("mindexerbase/sortfolder/*.mp4").expect("failed to find glob path") { // searches for mp4 files
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy(); // shortens path to rename files into unique name
                    println!("[i] indexing video: {}", path_shortened); // prints shortened path to let user see what's getting indexed
                    match fs::copy(&path, format!("mindexerbase/videos/{}", path_shortened)) { // copies videos in sortfolder to videos
                        Ok(_) => {
                            println!("[i] video indexed succesfully") // prints info if video got indexed succesfully
                        }
                        Err(e) => {
                            println!("[!] unable to index video: {}", e) // prints error if video didn't got indexed
                        }
                    }
                    match fs::remove_file(path) { // clean sortfolder
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully") // prints info if sortfolder got cleaned succesfully
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e) // prints error if sortfolder didn't got cleaned
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.mp3").expect("failed to find glob path") { // searches for mp3 files
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy(); // shortens path to rename files into unique name
                    println!("[i] indexing song: {}", path_shortened); // prints shortened path to let user see what's getting indexed
                    match fs::copy(&path, format!("mindexerbase/songs/{}", path_shortened)) { // copies songs in sortfolder to songs
                        Ok(_) => {
                            println!("[i] song indexed succesfully") // prints info if song got indexed succesfully
                        }
                        Err(e) => {
                            println!("[!] unable to index song: {}", e) // prints error if song didn't got indexed
                        }
                    }
                    match fs::remove_file(path) { // clean sortfolder
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully") // prints info if sortfolder got cleaned succesfully
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e) // prints error if sortfolder didn't got cleaned
                        }
                    }
                }
                Err(_) => {}
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
                        println!("[i] directories created succesfully") // prints info if it created mindexerbase succesfully
                    }
                    Err(e) => {
                        println!("[!] unable to create mindexerbase: {}", e) // prints error if it didn't created mindexerbase
                    }
                }
            }
        } else {
            println!("[i] aborting") // do nothing if user declined
        }
    }
}