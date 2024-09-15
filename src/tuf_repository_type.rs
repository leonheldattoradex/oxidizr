use std::fmt;

pub struct RepositoryType {
    type_: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Unknown = -1,
    Image = 0,
    Director = 1,
}

impl RepositoryType {
    pub const IMAGE: &'static str = "Image";
    pub const DIRECTOR: &'static str = "Director";

    pub fn new() -> Self {
        RepositoryType {
            type_: Type::Unknown,
        }
    }

    pub fn image() -> Self {
        RepositoryType { type_: Type::Image }
    }

    pub fn director() -> Self {
        RepositoryType {
            type_: Type::Director,
        }
    }

    pub fn from_int(type_val: i32) -> Self {
        let type_ = match type_val {
            0 => Type::Image,
            1 => Type::Director,
            _ => Type::Unknown,
        };
        RepositoryType { type_ }
    }

    pub fn from_str(repo_type: &str) -> Result<Self, String> {
        let type_ = if repo_type == Self::DIRECTOR {
            Type::Director
        } else if repo_type == Self::IMAGE {
            Type::Image
        } else {
            return Err(format!("Incorrect repo type: {}", repo_type));
        };
        Ok(RepositoryType { type_ })
    }

    pub fn to_string(&self) -> String {
        match self.type_ {
            Type::Director => Self::DIRECTOR.to_string(),
            Type::Image => Self::IMAGE.to_string(),
            _ => String::new(),
        }
    }
}

impl fmt::Display for RepositoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<RepositoryType> for String {
    fn from(repo_type: RepositoryType) -> Self {
        repo_type.to_string()
    }
}

impl From<RepositoryType> for i32 {
    fn from(repo_type: RepositoryType) -> Self {
        repo_type.type_ as i32
    }
}
