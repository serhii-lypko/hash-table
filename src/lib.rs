use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/*
    TODO:

    - generics ✅
    - resize ✅
    - delete ✅
    - impl iter
    - handle collision
*/

const DEFAULT_BUCKET_SIZE: usize = 100;

type Bucket<K, V> = Option<KV<K, V>>;
type Buckets<K, V> = Vec<Bucket<K, V>>;

#[derive(Clone, Debug)]
struct KV<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug)]
struct HashTable<K, V>
where
    K: Debug,
    V: Debug,
{
    buckets: Buckets<K, V>,
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    pub fn new(with_capacity: usize) -> Self {
        let buckets: Buckets<K, V> = vec![None; with_capacity];

        HashTable { buckets, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size() + 1 > self.buckets.len() {
            self.resize();
        }

        let kv = KV {
            key: key.clone(),
            value,
        };

        let index = self.create_index(key);

        self.buckets[index] = Some(kv);
        self.size += 1;
    }

    pub fn get(&self, key: K) -> Option<V> {
        let index = self.create_index(key.clone());
        self.buckets[index]
            .clone()
            .and_then(|kv| if kv.key == key { Some(kv.value) } else { None })
    }

    pub fn delete(&mut self, key: K) {
        let index = self.create_index(key);
        self.buckets[index] = None;
    }

    fn create_index(&self, key: K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash = s.finish();

        // Modulo arithmetic -> Uniform Distribution
        (hash % (self.buckets.len() as u64)) as usize
    }

    fn resize(&mut self) {
        let old_buckets = self.buckets.clone();
        self.buckets = vec![None; old_buckets.len() + DEFAULT_BUCKET_SIZE];

        for bucket in old_buckets {
            if let Some(bucket) = bucket {
                self.insert(bucket.key, bucket.value);
            }
        }
    }
}

impl<K, V> Iterator for HashTable<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hash_table: HashTable<String, u64> = HashTable::new(100);
        assert_eq!(hash_table.size(), 0);
    }

    #[test]
    fn test_insert() {
        let mut hash_table: HashTable<String, u64> = HashTable::new(100);

        hash_table.insert("key1".to_string(), 1);
        hash_table.insert("key2".to_string(), 2);
        hash_table.insert("key3".to_string(), 3);

        assert_eq!(hash_table.size(), 3);

        assert_eq!(hash_table.get("key1".to_string()), Some(1));
        assert_eq!(hash_table.get("key3".to_string()), Some(3));
    }

    #[test]
    fn test_delete() {
        let mut hash_table: HashTable<String, u64> = HashTable::new(100);

        hash_table.insert("key1".to_string(), 1);
        hash_table.insert("key2".to_string(), 2);
        hash_table.insert("key3".to_string(), 3);

        assert_eq!(hash_table.get("key2".to_string()), Some(2));

        hash_table.delete("key2".to_string());

        assert_eq!(hash_table.get("key2".to_string()), None);
    }

    #[test]
    fn test_iterator() {
        let mut hash_table = HashTable::new(10);
        hash_table.insert("key_1".to_string(), "value1".to_string());
        hash_table.insert("key_2".to_string(), "value2".to_string());

        let iter = hash_table.clone().into_iter();

        todo!()
    }

    #[test]
    fn test_resize() {
        let mut hash_table: HashTable<String, u64> = HashTable::new(3);

        assert_eq!(hash_table.buckets.len(), 3);

        hash_table.insert("key_1".to_string(), 1);
        hash_table.insert("key_22".to_string(), 2);
        hash_table.insert("key_33".to_string(), 3);

        hash_table.insert("key_4".to_string(), 4);

        assert_eq!(hash_table.buckets.len(), 103);

        assert_eq!(hash_table.get("key_1".to_string()), Some(1));
        assert_eq!(hash_table.get("key_22".to_string()), Some(2));
        assert_eq!(hash_table.get("key_33".to_string()), Some(3));
        assert_eq!(hash_table.get("key_4".to_string()), Some(4));
    }
}
