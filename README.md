# Tiler

⚠️ This repository is not actively developed due to time constraints.


## Overview

Tiler is a universal transpiler designed to work seamlessly with all programming languages. Leveraging UParseTree and a set of transpilation rules known as "Tiles," Tiler provides a versatile solution for code conversion.

## Key Features

- **Universal Compatibility:** Tiler aims to create a single library that can transpile code across various programming languages.

- **Cross-Platform UI Development:** Build application UIs once and deploy them on multiple platforms such as Flutter, React, and Android Studio.

- **Efficient Code Conversion:** Utilize Tiler's transpilation capabilities to convert code between programming languages effortlessly.

## Benefits

- **Simplified Development:** With Tiler, you can streamline the development process by maintaining one library for all supported languages.

- **Cross-Platform UIs:** Develop user interfaces once and deploy them on different platforms, reducing development time and effort.

- **Effortless Code Conversion:** Tiler simplifies the process of converting code between programming languages, promoting code interoperability.

## Dependencies

Tiler relies on the following Rust dependencies:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
walkdir = "2.3.2"
regex = "1.7"
quote = "1.0"
fancy-regex = "0.10.0"
syn = { version = "1.0", features = ["parsing", "full", "extra-traits"] }
```

Additionally, Tiler uses the `fancy_regex` crate for transpilation and rule validation.

## Usage

### Transpiler Example

The provided code demonstrates a transpiler function using Tiler and UParseTree. The `find` function showcases the application of transpilation rules to Rust code.

```rust
// Code snippet for transpilation using Tiler and UParseTree
use fancy_regex::Regex;
use walkdir::WalkDir;

pub fn find() {
    // Transpilation rules using fancy_regex and UParseTree
    let re = Regex::new("^(?!.*(\"(attr|green|blue).rs\")).*(\"(([a-zA-Z0-9_])+).rs\")").unwrap();
    let re_struct = Regex::new("pub\\s+struct\\s+([A-Z][A-Za-z0-9]*)").unwrap();

    // Iterate through files in the specified directory
    for entry in WalkDir::new("sample/syn").max_depth(1).into_iter().filter_map(|e| e.ok()) {
        // Check if the file name matches the transpilation rule
        if re.is_match(&format!("{:?}", entry.file_name())).unwrap() {
            // Perform transpilation on the file's content
            // ...

            // Write the transpiled code to a new file
            // ...
        }
    }
}

#[allow(unused)]
pub fn main() {
    // Execute the transpiler function
    find();
    println!("Successfully done");
}
```

### Other Examples

Tiler also includes examples of additional functionalities, such as AST creation and serialization.


Feel free to explore the functionalities provided by Tiler for efficient code conversion across various programming languages. For more information, refer to the UParseTree repository for the universal parser tree implementation.