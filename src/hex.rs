pub fn validate_hex(hexstr: &str) -> Option<&str> {
    let inputlength: usize = hexstr.len();
    if inputlength != 7 {
        None
    }
    else{
        let hexchars = hexstr.chars();
        let revhexchars = hexchars.rev();
        if hexstr.chars().collect::<Vec<char>>()[0] == '#' && revhexchars
            .take(inputlength - 1)
            .filter(|x| x.is_alphanumeric())
            .collect::<String>()
            .len() == inputlength - 1 {
            Some(hexstr)
        }
        else {
            None
        }
    }
}
