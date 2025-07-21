use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct User {
    // pub id: usize,
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Clone, Default)]
pub enum Role {
    Admin,
    #[default]
    User,
}

// 作用：将枚举类型转换为字符串
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Movie {
    pub disc: usize,
    pub year: String,
    pub title: String,
    pub remark: Option<String>,
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.disc == other.disc && self.year == other.year && self.title == other.title
    }
}
