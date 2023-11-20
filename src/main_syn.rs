// use std::env;
// use std::fs::File;
// use std::io::Read;
// use std::process;

// pub fn main() {
//     let mut args = env::args();
//     let _ = args.next(); // executable name

//     let filename = match (args.next(), args.next()) {
//         (Some(filename), None) => filename,
//         _ => {
//             eprintln!("Usage: dump-syntax path/to/filename.rs");
//             process::exit(1);
//         }
//     };

//     let mut file = File::open(&filename).expect("Unable to open file");

//     let mut src = String::new();
//     file.read_to_string(&mut src).expect("Unable to read file");

//     let syntax = syn::parse_file(&src).expect("Unable to parse file");

//     // Debug impl is available if Syn is built with "extra-traits" feature.
//     println!("{:#?}", syntax);
// }
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path;
use std::io::Read;

fn run() {
    let mut file = fs::File::open("sample.rs").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let ast: syn::File = syn::parse_file(&content).unwrap();
    // if let Some(shebang) = ast.shebang {
    //     println!("{}", shebang);
    // }
    let code = format!("{:?}", &ast);
    let path = path::Path::new("sample-ast.rs");
        
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = fs::File::create(&path).unwrap();

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    file.write_all(code.as_bytes()).unwrap();
    println!("done successfully!");


    // write_and_fmt("stmt.rs", quote::quote!(#&ast)).expect("unable to save or format");
}

// use syn::{Expr, Result};

// fn run2() -> Result<()> {
//     let code = "assert_eq!(u8::max_value(), 255)";
//     let expr = syn::parse_str::<Expr>(code)?;
//     println!("{:#?}", expr);
//     Ok(())
// }


// use std::fs;
// use std::io;
// use std::path::Path;
// use std::process::Command;


// fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> io::Result<()> {
//     fs::write(&path, code.to_string())?;

//     Command::new("rustfmt")
//         .arg(path.as_ref())
//         .spawn()?
//         .wait()?;

//     Ok(())
// }

pub fn main() {
    run();
}

