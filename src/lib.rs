use std::io::BufRead;

pub fn yeet<R: BufRead>(reader: &mut R) {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    assert!(!input.is_empty());
}
