use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Directory {
    pub files: Vec<String>,
    pub child_directories: Vec<Box<Directory>>,
    pub(crate) size: u64,
    pub(crate) directory_name: String,
}

impl Eq for Directory {}

impl PartialEq<Self> for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.directory_name == other.directory_name && self.size == other.size
    }
}

impl PartialOrd<Self> for Directory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self == other {
            true => Some(Ordering::Equal),
            false => {
                if self.size < other.size {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let d1 = Directory {
            files: vec![],
            child_directories: vec![],
            size: 100,
            directory_name: "test_di".to_string(),
        };

        assert_eq!(d1, d1);
        let d2 = Directory {
            files: vec![],
            child_directories: vec![],
            size: 100,
            directory_name: "test_di".to_string(),
        };
        assert_eq!(d1, d2);
    }
    #[test]
    fn test_less() {
        let d1 = Directory {
            files: vec![],
            child_directories: vec![],
            size: 100,
            directory_name: "test_di".to_string(),
        };
        let d2 = Directory {
            files: vec![],
            child_directories: vec![],
            size: 52,
            directory_name: "test_di12".to_string(),
        };
        assert!(d2 < d1);
    }
}
