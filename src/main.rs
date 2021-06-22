use std::collections::HashMap;
use std::env;
use std::io::{stdout, Error, ErrorKind, Write};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let id = args.get(1);

    return match id {
        Some(id) => {
            let license_text = license_text(id);

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

fn build_custom_licenses() -> HashMap<String, &'static [u8]> {
    let mut custom_licenses = HashMap::<String, &[u8]>::new();

    custom_licenses.insert(
        "LicenseRef-ACAB-1.0".to_string(),
        include_bytes!("../licenses/LicenseRef-ACAB-1.0"),
    );

    custom_licenses
}

fn license_text(id: &str) -> std::result::Result<String, ()> {
    let custom_licenses = build_custom_licenses();

    return create_custom_license_text(custom_licenses, id).or(create_license_text(id));
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

fn create_custom_license_text(
    custom_license_map: HashMap<String, &[u8]>,
    custom_license: &str,
) -> std::result::Result<String, ()> {
    match custom_license_map.get(custom_license) {
        Some(str) => {
            return Ok(String::from_utf8_lossy(str).to_string());
        }

        None => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_valid_contents() {
        let contents = create_license_text(&"MIT");

        assert!(contents.is_ok());
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

    #[test]
    fn should_support_custom_licenses() {
        let mut custom_licenses = HashMap::<String, &[u8]>::new();
        custom_licenses.insert("LicenseRef-Test".to_string(), b"Test license");
        let contents = create_custom_license_text(custom_licenses, "LicenseRef-Test");

        assert!(contents.is_ok());
        assert!(contents.unwrap() == "Test license")
    }

    #[test]
    fn should_fail_on_non_existant_custom_licenses() {
        let mut custom_licenses = HashMap::<String, &[u8]>::new();
        custom_licenses.insert("LicenseRef-Test".to_string(), b"Test license");
        let contents = create_custom_license_text(custom_licenses, &"license does not exist");

        assert!(contents.is_err());
    }

    #[test]
    fn should_create_custom_licenses() {
        let contents = license_text(&"LicenseRef-ACAB-1.0");

        assert!(contents.is_ok());
    }
}
