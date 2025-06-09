pub fn is_valid_url(link: &str) -> bool {
    if link.contains("w/index.php") {
        return false;
    }

    if link.contains('%') {
        return false;
    }

    for ch in link.chars() {
        if ch as u32 > 127 {
            return false;
        }

        if !ch.is_ascii_alphanumeric() && !is_allowed_symbol(ch) {
            return false;
        }
    }

    true
}

fn is_allowed_symbol(ch: char) -> bool {
    const ALLOWED: &str = "-._~:/?#[]@!$&'()*+,;=";
    ch.is_ascii_graphic() || ALLOWED.contains(ch)
}
