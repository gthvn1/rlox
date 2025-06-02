pub fn report_error_at(line: usize, at: &str, msg: &str) {
    eprintln!("[line {}] Error {}: {}", line, at, msg);
}

pub fn report_error(line: usize, msg: &str) {
    report_error_at(line, "", msg);
}
