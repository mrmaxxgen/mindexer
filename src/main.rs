/*mindexer (c) mrmaxxgen 2026
 *licensed under GNU GPLv3 or later <https://www.gnu.org/licenses/>
 */
use std::io;
use std::process::Command;

fn main() {
    
    /*mindexer's purpose is very simple: to sort all files in one big folder called "sortfolder" into multiple smaller folder
     *for file type like "videos", "songs", "repos" ecc... it works by spawning shells commands that moves each file into a
     *specific folder by it's type, like "song.mp3" from "sortfolder" directory to "songs" directory.
     *if mindexer doesn't detects the base, it will create it, or, if it's not running inside the base, it will simply
     *outputs an error message and quit. throughout the base making and file sorting, each file and base modification will
     *be logged and reported on the terminal like "[ok] sorting file <FILE> into folder <FOLDER>". that's all! :3
     *for compilation and usage instructions, see README.MD or mindexer github page <https://github.com/mrmaxxgen/mindexer.git>
    */

    /*steps: -check if mindexerbase exists. if not, ask user to create it.
     *       -check if command is running inside mindexerbase. if not, exit.
     *       -move videos from "sortfolder" dir to "videos" dir.
     *       -move songs from "sortfolder" dir to "songs" dir.  
    */
    
}