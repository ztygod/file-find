pub fn print_error(msg: impl Into<String>) {
    eprintln!("[file-find error]: {}", msg.into())
}
