use std::fs::File;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::ffi::OsString;
//decide on save format
//just use 1 and 0 seperated by whitespace

#[cfg(unix)]
const LINE_END: &'static str = "\n";

#[cfg(windows)]
const LINE_END: &'static str = "\r\n";

const FALSE: &'static str = ".";
const TRUE: &'static str = "O";

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
        TRUE => Some(true),
        FALSE => Some(false),
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

pub fn save(data: &Vec<Vec<bool>>) {
    //make save dir if not existing yet
    //get all files in savedir that contain savename
    //sort names
    //make new file by incremented highest
    //filenames will be in save<num>.seed format.
    let mut entries = get_save_entries();
    entries.sort();

    let mut newsave: File;
    let save_num: usize;
    if entries.is_empty() {
        save_num = 0;
    }
    else {
        let last_save = entries.last()
            .unwrap()
            .to_str()
            .unwrap();
        save_num = last_save.chars()
            .skip(4)
            .take(last_save.len() - 9)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }
    let mut cwd = env::current_dir().unwrap();
    let mut savecwd = cwd.to_owned();
    savecwd.push("saves");
    env::set_current_dir(&savecwd).expect("Failed to cd to save directory.");
    newsave = File::create(format!("save{}.seed", save_num + 1))
        .expect("Failed to open file for writing");
    newsave.write_all(data_to_string(data).as_bytes()).expect("Save write failed.");
    env::set_current_dir(&cwd).expect("Failed to cd to root directory.");
}

fn get_save_entries() -> Vec<OsString> {
    let mut cwd = env::current_dir().unwrap();
    let mut savecwd = cwd.to_owned();
    savecwd.push("saves");
    fs::create_dir_all(&savecwd).expect("Failed to make save directory.");
    env::set_current_dir(&savecwd).expect("Failed to cd to save directory.");
    let mut entries = fs::read_dir(&savecwd)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .filter(|entry| {let entrystr = entry.to_str().unwrap();
                    entrystr.contains(".seed") && entrystr.contains("save")})
            .collect::<Vec<OsString>>();
    env::set_current_dir(&cwd).expect("Failed to cd to root directory.");
    entries
}

fn data_to_string(data: &Vec<Vec<bool>>) -> String {
    let mut serialized: String = "".to_string();
    for row in data {
        serialized = serialized + row.iter()
                                     .map(|v| bool_to_str(v))
                                     .collect::<Vec<&str>>()
                                     .join(" ")
                                     .to_owned()
                                     .as_str() + LINE_END;
    }
    serialized = serialized.chars()
            .take(serialized.len() - LINE_END.len())
            .collect::<String>();
    serialized
}

fn bool_to_str(data: &bool) -> &'static str {
    if *data {
        TRUE
    } 
    else {
        FALSE
    }
} 

pub fn clear_saves() {
    for file in get_save_entries() {
        fs::remove_file(&file).expect("Failed to delete save file.");
    }
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
