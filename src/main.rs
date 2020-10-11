use std::env;
use std::io::{stdout, Error, ErrorKind, Write};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let id = args.get(1);

    return match id {
        Some(id) => {
            let license_text = create_license_text(id);

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

fn create_license_text(id: &str) -> std::result::Result<String, ()> {
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
    fn should_create_valid_contents() {
        let contents = create_license_text(&"MIT");

        assert!(contents.is_ok())
    }

    #[test]
    fn should_error_invalid_contents() {
        let contents = create_license_text(&"mit");

        assert!(contents.is_err());
    }

    #[test]
    fn should_add_to_stdout() {
        let mut stdout = Vec::new();
        print_license_contents(&mut stdout, String::from("sup")).unwrap();

        assert_eq!(stdout, b"sup");
    }
}
