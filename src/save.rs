use std::fs::File;

pub fn parse(filepath: &str) -> Vec<Vec<bool>> {
    //will be a commandline argument
    let f = try!(File::open(filepath));
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
