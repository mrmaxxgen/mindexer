// mindexer (c) mrmaxxgen 2026
// licensed under the MIT license <https://mit-license.org>

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

    // structure: 
    //            - core function (files indexing)
    //            - base creator

    // issues: 
    //            TODO: code repeats too often, maybe invent a sorting algorithm to prevent that

    if Path::new("mindexerbase").is_dir() { 
        for entry in glob("mindexerbase/sortfolder/*.mp4").expect("failed to find glob path") { 
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy(); 
                    println!("[i] indexing video: {}", path_shortened); 
                    match fs::copy(&path, format!("mindexerbase/videos/{}", path_shortened)) { 
                        Ok(_) => {
                            println!("[i] video indexed succesfully") 
                        }
                        Err(e) => {
                            println!("[!] unable to index video: {}", e) 
                        }
                    }
                    match fs::remove_file(path) { 
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully") 
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e) 
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.mov").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing video: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/videos/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] video indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index video: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.mp3").expect("failed to find glob path") { 
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy(); 
                    println!("[i] indexing song: {}", path_shortened); 
                    match fs::copy(&path, format!("mindexerbase/songs/{}", path_shortened)) { 
                        Ok(_) => {
                            println!("[i] song indexed succesfully") 
                        }
                        Err(e) => {
                            println!("[!] unable to index song: {}", e) 
                        }
                    }
                    match fs::remove_file(path) { 
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully") 
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e) 
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.png").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing image: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/images/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] image indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index image: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.jpg").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing image: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/images/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] image indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index image: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.jpeg").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing image: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/images/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] image indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index image: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.img").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing image: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/images/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] image indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index image: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.iso").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing ISO: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/ISOs/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] ISO indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index ISO: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
        for entry in glob("mindexerbase/sortfolder/*.zip").expect("failed to find glob pattern") {
            match entry {
                Ok(path) => {
                    let path_shortened = path.file_name().unwrap().to_string_lossy();
                    println!("[i] indexing image: {}", path_shortened);
                    match fs::copy(&path, format!("mindexerbase/zips/{}", path_shortened)) {
                        Ok(_) => {
                            println!("[i] zip indexed succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to index zip: {}", e)
                        }
                    }
                    match fs::remove_file(path) {
                        Ok(_) => {
                            println!("[i] sortfolder cleaned succesfully")
                        }
                        Err(e) => {
                            println!("[!] unable to clean sortfolder: {}", e)
                        }
                    }
                }
                Err(_) => {}
            }
        }
    } else {
        println!("[?] mindexerbase not found, create it? (y/n):");
        let mut base_maker_ask = String::new();
        io::stdin().read_line(&mut base_maker_ask).expect("failed to read line");
        if base_maker_ask.trim() == "y" {
            let basedirs = ["mindexerbase/sortfolder", "mindexerbase/videos", "mindexerbase/songs", "mindexerbase/images", "mindexerbase/ISOs", "mindexerbase/zips"];
            for dir in basedirs {
                match fs::create_dir_all(dir) { 
                    Ok(_) => {
                        println!("[i] directories created succesfully")
                    }
                    Err(e) => {
                        println!("[!] unable to create mindexerbase: {}", e)
                    }
                }
            }
        } else {
            println!("[i] aborting")
        }
    }
}
