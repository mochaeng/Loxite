pub fn error(line: usize, message: &str) {
    report(line, "".to_string(), message)
}

fn report(line: usize, where_in: String, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_in, message);
    // self.had_error = true;
}
