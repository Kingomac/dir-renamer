use std::fmt;

pub struct FileInfo {
    pub name: String,
    pub extension: String,
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.extension)
    }
}

impl FileInfo {
    pub fn to_string(self) -> String {
        return vec![self.name, self.extension].join("");
    }
}
