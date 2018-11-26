// Copyright 2018 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// you can obtain one at http://mozilla.org/MPL/2.0/.


#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;


const VERSION: &'static str = "tb 0.1.0
Copyright (C) 2018 Nathan Sizemore <nathanrsizemore@gmail.com>
License: MPL-2.0 https://www.mozilla.org/en-US/MPL/2.0
This is free software: you are free to change and redistribute it.";

const USAGE: &'static str = "
tb - generates a text blob of given size. Size is in bytes by default.

Usage:
    tb [-k] <size>
    tb -h | --help
    tb -v | --version

Options:
    -k, --kilobytes    Size is in kilobytes.
    -m, --megabytes    Size is in megabytes.
    -h, --help         Show this screen.
    -v, --version      Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_size: u32,
    flag_kilobytes: bool,
    flag_megabytes: bool,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}", VERSION);
        return;
    }

    let mut size_in_bytes = args.arg_size;
    if args.flag_kilobytes {
        size_in_bytes *= 1024;
    } else if args.flag_megabytes {
        size_in_bytes = size_in_bytes * 1024 * 1024;
    }

    let mut blob = String::with_capacity(size_in_bytes as usize);
    for _ in 0..size_in_bytes {
        blob.push('0');
    }

    println!("{}", blob);
}
