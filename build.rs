/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use std::io::{BufRead, BufReader};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let uni_data = File::open("UnicodeDataFixed.txt").unwrap();

    let mut builder = phf_codegen::Map::new();

    for line in BufReader::new(uni_data).lines() {
        let l = line.unwrap();
        let mut data = l.split(';');
        let num = u32::from_str_radix(data.next().unwrap(), 16).unwrap();
        let name = data.next().unwrap();

        builder.entry(num, &format!("\"{}\"", name));
    }

    writeln!(
        &mut file,
        "static UNICODE: ::phf::Map<u32, &'static str> = \n{};",
        builder.build(),
    )
    .unwrap();
}
