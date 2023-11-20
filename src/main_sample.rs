use std::{fs::File, path::{Path}, io::{Read, self, BufRead, Write}};
// use regex::Regex;
use fancy_regex::Regex;

use walkdir::WalkDir;

pub fn find() {
    let re = Regex::new("^(?!.*(\"(attr|green|blue).rs\")).*(\"(([a-zA-Z0-9_])+).rs\")").unwrap();
    let re_struct = Regex::new("pub\\s+struct\\s+([A-Z][A-Za-z0-9]*)").unwrap();

    for entry in WalkDir::new("sample/syn").max_depth(1).into_iter().filter_map(|e| e.ok()) {
    // for entry in WalkDir::new("../syn/src").max_depth(1).into_iter().filter_map(|e| e.ok()) {

        let fname = format!("{:?}", entry.file_name());
        // println!("{}", fname);

        if re.is_match(&fname).unwrap() {
            let path = format!("{}", entry.path().display());
            let file = File::open(Path::new(&path)).unwrap();
            
            let lines = io::BufReader::new(file).lines();

            let mut code = String::new();

            for line in lines {
                let line_text = line.unwrap();
                if re_struct.is_match(&line_text).unwrap() {
                    code.push_str("#[derive(Debug)]\n");
                }
                code.push_str(&line_text);
                code.push_str("\n");
            }

            let path = Path::new("lorem_ipsum.rs");
        
            // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = File::create(&path).unwrap();
        
            // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
            file.write_all(code.as_bytes()).unwrap();

            // file.read_to_string(&mut code).unwrap();

        }
        // println!("{}", entry.path().display());
    }
}

#[allow(unused)]
pub fn main() {
    find();
    println!("successfuly done")
    // let mut file = File::open(Path::new("sample/syn/stmt.rs")).unwrap();
    // let mut code = String::new();
    
    // file.read_to_string(&mut code).unwrap();
    // println!("{}", code);
}
