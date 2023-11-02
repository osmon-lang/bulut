use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    /// Name of the project
    name: String,

    /// Version of the package
    version: String,

    /// Description of the package
    description: String,

    /// Version of Osmon std library
    std: String,
}

pub enum PackageError {
    /// Instance can't be parsed to TOML
    NotParsable,

    /// Instance can't be written to file
    NotWritable
}

impl Package {
    pub fn new(
        name: String,
        version: String,
        description: String,
        std: String
    ) -> Self {
        Self {
            name, version, description, std
        }
    }

    pub fn from_file(path: &str) -> Self {
        let file = std::fs::read_to_string(path).unwrap();
        let package: Package = toml::from_str(&file).unwrap();

        package
    }

    pub fn save(&self, path: &str) -> Result<(), PackageError> {
        let convert = match toml::to_string_pretty(self) {
            Ok(convert) => convert,
            Err(_) => return Err(PackageError::NotParsable)
        };

        // Save package to file
        match std::fs::write(path, convert) {
            Ok(_) => Ok(()),
            Err(_) => Err(PackageError::NotWritable)
        }
    }
}