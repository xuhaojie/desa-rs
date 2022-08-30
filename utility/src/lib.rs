pub mod download;
pub mod process;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn centre_print(text: &str, ch: char, width: usize) {
    let mut prefix = String::with_capacity(width);
    let length = (width - text.len()) / 2;
    for _ in 0..length {
        prefix.push(ch);
    }

    let mut surfix = String::with_capacity(width);
    let length = width - prefix.len() - text.len();
    for _ in 0..length {
        surfix.push(ch);
    }
    println!("{}{}{}", prefix, text, surfix);
}
