pub type Timestamp = u64;

#[derive(PartialEq, Eq, PartialOrd, Clone, Ord, Debug)]
pub struct Key(pub Vec<u8>);
#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Value(pub Vec<u8>);
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub value: Option<Value>,
    pub timestamp: Timestamp,
}

impl Entry {
    pub fn new(value: Value, timestamp: Timestamp) -> Self {
        Self {
            value: Some(value),
            timestamp,
        }
    }
    //O'chirilgan (tombstone) yozuv yaratish
    pub fn tombstone(timestamp: Timestamp) -> Self {
        Self {
            value: None,
            timestamp,
        }
    }

    pub fn is_tombstone(&self) -> bool {
        self.value.is_none()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_equal_and_ordering() {
        let k1 = Key(vec![1, 2, 3]);
        let k2 = Key(vec![1, 2, 3]);
        let k3 = Key(vec![1, 2, 4]);

        assert_eq!(k1, k2);

        assert!(k1 < k3);
    }

    #[test]
    fn test_tombstone_entry() {
        let key = Key(vec![10, 20]);
        let entry = Entry::tombstone(1);

        assert!(entry.is_tombstone());
        assert_eq!(entry.value, None);
        assert_eq!(entry.timestamp, 1);
    }
}
