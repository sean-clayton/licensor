use std::env;
use std::io::{stdout, Error, ErrorKind, Write};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let id = args.get(1);

    return match id {
        Some(id) => {
            let license_text = create_contents(id);

            match license_text {
                Ok(license_text) => return print_license_contents(&mut stdout(), license_text),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Must provide a valid SPDX identifier.",
                )),
            }
        }

        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Must provide a valid SPDX identifier.",
            ))
        }
    };
}

fn print_license_contents(stdout: &mut dyn Write, contents: String) -> std::io::Result<()> {
    return write!(stdout, "{}", contents);
}

fn create_contents(id: &str) -> std::result::Result<String, ()> {
    let license = license::from_id(id);

    match license {
        Some(license) => {
            return Ok(String::from(license.text()));
        }

        None => return Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_contents() {
        let contents = create_contents(&"MIT");

        assert!(contents.is_ok())
    }

    #[test]
    fn test_print_license_contents() {
        let mut stdout = Vec::new();

        assert!(print_license_contents(&mut stdout, String::from("sup")).is_ok());
    }
}
