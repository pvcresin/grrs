use std::io::Write;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor si amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
