use std::process::ExitCode;

/// What `--version` prints. The version is read from the crate rather than written here, because
/// ADR-0068 requires it to be exact and to live in a file at the commit.
fn version_line() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

const USAGE: &str = "usage: beaket --version";

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("{}", version_line());
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_read_from_the_crate_and_not_written_in_the_source() {
        let line = version_line();
        assert_eq!(line, format!("beaket {}", env!("CARGO_PKG_VERSION")));
        // The literal below is the one this test exists to catch: if someone hard-codes a version
        // string, the crate can be bumped without the binary saying so.
        assert!(line.starts_with("beaket "));
        assert!(!line.contains("0.0.0-dev"));
    }
}
