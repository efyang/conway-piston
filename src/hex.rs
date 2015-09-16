pub fn validate_hex(hexstr: &str) -> Option<&str> {
    let inputlength: usize = hexstr.len();
    if inputlength != 6 {
        None
    }
    else{
        let hexchars = hexstr.chars();
        let revhexchars = hexchars.rev();
        if revhexchars
            .filter(|x| x.is_alphanumeric())
            .collect::<String>()
            .len() == inputlength {
            Some(hexstr)
        }
        else {
            None
        }
    }
}
