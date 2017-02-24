# typographic-linter
[![Build Status](https://travis-ci.org/rlustin/typographic-linter.svg?branch=master)](https://travis-ci.org/rlustin/typographic-linter)

Typographic linter – because we do care very much about typography.

## Overview

`typographic-linter` is Rust library that checks for common typographic rules in several languages:
English, French, German, Italian and Spanish.

## Library in action

Add the dependency in your `Cargo.toml`:
```toml
typographic_linter = { git = "https://github.com/rlustin/typograhic-linter" }
```

```rust
extern crate typographic_linter;

use typographic_linter::Linter;

fn main() {
    let linter = Linter::new("en".to_string()).unwrap();

    let content = "It's me...";
    let result = linter.check(content);

    if result.is_err() {
        let warnings = result.err().unwrap();

        println!("There are {} typographic warnings in “{}”:", warnings.len(), content);

        for warning in &warnings {
            println!("- At {}, {}: {}", warning.start, warning.end, warning.message);
        }
    } else {
        println!("There’s no typographic warning.");
    }
}
```

## Implemented rules

This library is a work in progress. For now, it only checks for the rules bellow.

### All languages
- curly apostrophes;
- ellipsis symbol;
- no space before comma;
- typographic quotation marks.

### French
- spaces before double punctionation marks.
