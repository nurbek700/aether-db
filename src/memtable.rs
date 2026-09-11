use crate::entry::{Entry, Key, Timestamp, Value};
use std::collections::BTreeMap;
pub struct MemTable {
    map: BTreeMap<Key, Entry>,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: Key, value: Value, ts: Timestamp) {
        let entry = Entry::new(value, ts);
        self.map.insert(key, entry);
    }

    pub fn delete(&mut self, key: Key, ts: Timestamp) {
        let tombstone = Entry::tombstone(ts);
        self.map.insert(key, tombstone);
    }

    pub fn get(&self, key: &Key) -> Option<&Entry> {
        self.map.get(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn approximate_size_bytes(&self) -> usize {
        self.map
            .iter()
            .map(|(k, entry)| {
                let key_size = k.0.len();
                let value_size = match &entry.value {
                    Some(v) => v.0.len(),
                    None => 0,
                };
                key_size + value_size
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    // use crate::memtable;

    use super::*;

    #[test]
    fn test_duplicate_insert_() {
        let mut memtable = MemTable::new();

        let key = b"user_101".to_vec();
        let value = b"nurbek_jon".to_vec();

        memtable.insert(Key(key.clone()), Value(value), 1000);
        assert_eq!(memtable.len(), 1);

        let value2 = b"malikov_X".to_vec();

        memtable.insert(Key(key.clone()), Value(value2), 1001);
        assert_eq!(memtable.len(), 1);

        assert_eq!(
            memtable.get(&Key(key.clone())).unwrap().value,
            Some(Value(b"malikov_X".to_vec()))
        )
    }

    #[test]
    fn delete_value_in_tree() {
        let mut memtable = MemTable::new();

        let key = b"user_01".to_vec();
        let value = b"user_nurbek".to_vec();

        memtable.insert(Key(key.clone()), Value(value), 1002);
        memtable.delete(Key(key.clone()), 1002);

        assert!(memtable.get(&Key(key.clone())).is_some());

        assert_eq!(
            memtable.get(&Key(key.clone())).unwrap().is_tombstone(),
            true
        );
        assert_eq!(memtable.get(&Key(key.clone())).unwrap().timestamp, 1002)
    }

    #[test]
    fn test_empty_memtable_get_return_none() {
        let memtable = MemTable::new();
        let key = b"non_existing".to_vec();

        assert_eq!(memtable.get(&Key(key)), None);
    }

    #[test]
    fn test_btreemap_ordering() {
        let mut memtable = MemTable::new();

        let key1 = vec![5, 0, 0];
        let key2 = vec![1, 0, 0];
        let key3 = vec![3, 0, 0];

        let val1 = vec![1, 2, 3];
        let val2 = vec![3, 4, 5];
        let val3 = vec![6, 7, 8];

        memtable.insert(Key(key1), Value(val1), 100);
        memtable.insert(Key(key2), Value(val2), 101);
        memtable.insert(Key(key3), Value(val3), 102);

        // cloned() metodi elementlari havola bo'lgan iteratorlar ustida ishlaydi. Har bir T clone trait'ni impl qilgan bo'lishi shart. Har bir iterator ichidagi &T ni T ga o'girish uchun ishlatiladi. collect()::<Vec<T>> bu yirda collect qiymatlarni bitta joyga yig'ish, :: bu esa turbofish - Rust kompilyatorga aniq tur haqida aniq ko'rsatma berish.
        let sorted_keys = memtable.map.keys().cloned().collect::<Vec<_>>();

        assert_eq!(
            vec![Key(vec![1, 0, 0]), Key(vec![3, 0, 0]), Key(vec![5, 0, 0])],
            sorted_keys
        );
    }
}
