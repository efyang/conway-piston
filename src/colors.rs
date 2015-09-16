pub fn validate_hex(hexstr: &str) -> bool {
    let inputlength: usize = hexstr.len();
    if inputlength != 7 {
        return false;
    }
    else{
        let hexchars = hexstr.chars();
        let revhexchars = hexchars.rev();
        hexstr.chars().collect::<Vec<char>>()[0] == '#' && revhexchars
            .take(inputlength - 1)
            .filter(|x| x.is_alphanumeric())
            .collect::<String>()
            .len() == inputlength - 1
    }
}
