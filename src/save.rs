use std::fs::File;
use std::io::prelude::*;

pub fn parse(filepath: &str) -> Vec<Vec<bool>> {
    //will be a commandline argument
    let mut f = File::open(filepath).unwrap();
    let mut data: String = String::new();
    f.read_to_string(&mut data);
    data = data.trim().to_string();
    unimplemented!();
}

pub fn save(filepath: &str, data: Vec<Vec<bool>>) {
    //make save dir if not existing yet
    //get all files in savedir that contain savename
    //sort names
    //make new file by incremented highest
    unimplemented!();
}

pub fn clear_saves(savedir: &str) {
    unimplemented!();
}
