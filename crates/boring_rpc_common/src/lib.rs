pub fn to_lower_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_lowercase());
    }
    buf
}

pub fn to_upper_camel_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut upper = true;
    for c in s.chars() {
        if c == '_' {
            upper = true;
            continue;
        }
        if upper {
            buf.push(c.to_ascii_uppercase());
        } else {
            buf.push(c);
        }
        upper = false;
    }
    buf
}
