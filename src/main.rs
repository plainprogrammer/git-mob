/*
 *  Copyright (c) 2020 - James Thompson <james@thomps.onl>
 *  Licensed under the MIT License as found in LICENSE file.
 */
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let _matches = App::from_yaml(yaml).get_matches();
}
