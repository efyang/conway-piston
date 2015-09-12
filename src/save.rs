use std::fs;
use std::env;
use std::io::prelude::*;
//decide on save format
//just use 1 and 0 seperated by whitespace

pub fn parse(filepath: &str) -> Vec<Vec<bool>> {
    //will be a commandline argument
    let mut f = File::open(filepath).expect("Invalid file path.");
    let mut strdata: String = String::new();
    f.read_to_string(&mut strdata).expect("Data read failure.");
    let mut data: Vec<Vec<bool>> = Vec::new();
    for line in strdata.lines() {
        data.push(line.split_whitespace()
                      .map(|c| str_to_bool(c).expect("Invalid data."))
                      .collect::<Vec<bool>>());
    }
    if is_valid(&data) {
        data
    } 
    else {
        panic!("Invalid data.")
    }
}

fn str_to_bool(val: &str) -> Option<bool> {
    match val {
        "0" => Some(false),
        "1" => Some(true),
        _ => None,
    }
}

fn all_same(data: &Vec<usize>) -> bool {
    let mut cleandata = data.clone();
    cleandata.dedup();
    cleandata.pop();

    cleandata.is_empty()
}

fn is_valid(data: &Vec<Vec<bool>>) -> bool {
    let widths: Vec<usize> = data.iter()
        .map(|row| row.len())
        .collect::<Vec<usize>>();
    all_same(&widths)
}

pub fn save(filepath: &str, data: Vec<Vec<bool>>) {
    //make save dir if not existing yet
    //get all files in savedir that contain savename
    //sort names
    //make new file by incremented highest
    let cwd = env::current_dir().unwrap();
    cwd.push(saves);
    fs::create_dir_all(cwd);
    env::set_current_dir(&cwd);
    

    unimplemented!();
}

fn data_to_string(data: Vec<Vec<bool>>) -> String {
    
}

pub fn clear_saves(savedir: &str) {
    unimplemented!();
}


#[cfg(test)]
pub mod test {
    //#[test]
    //fn parse() {
        //assert_eq()
    //}
    #[test]
    fn all_same_test() {
        assert!(super::all_same(&vec![20,20,20,20,20]));
        assert!(!super::all_same(&vec![21,20,20,20]));
        assert!(super::all_same(&Vec::new()));
    }
}
