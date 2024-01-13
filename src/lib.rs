use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/*
    Hash Function: The hash table uses a hash function to convert the key into a hash code,
    which is usually an integer. The hash code is then converted to an index into the array.
    The hash function is designed to ensure that different keys map to different indexes.
    However, in some cases, two different keys might produce the same hash code, leading to a collision.

    Handling Collisions: There are several methods for handling collisions. Two of the most common
    are separate chaining and open addressing. In separate chaining, each bucket is independent,
    and contains a list of entries for each index. If a collision occurs, the entry is just added
    to the list at that index. In open addressing, if a collision occurs, we find the next available
    slot or bucket.

    Insertion: When inserting a new key-value pair, the hash table first applies the hash function
    to the key and gets the hash code. It then converts this hash code into an index and stores
    the key-value pair in the bucket at that index.

    Lookup: When looking up a key, the hash table does the same thing: it applies the hash function,
    converts the hash code to an index, and searches the bucket at that index for the key.
    If the key is found, its corresponding value is returned.

    Deletion: Deletion is similar to lookup. The hash table applies the hash function to the key,
    converts the hash code to an index, and removes the key-value pair from the bucket at that index.

    Resizing: If the hash table becomes too full (i.e., the load factor is too high), it needs to be
    resized to maintain performance. Resizing involves creating a new, larger array and rehashing
    all the current keys into it.
*/

/*
    TODO:

    - generics ✅
    - resize ✅
    - delete, keys
    - collision
*/

const DEFAULT_BUCKET_SIZE: usize = 100;

type Buckets<K, V> = Vec<Option<KV<K, V>>>;

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

    pub fn delete(&mut self, key: String) {
        todo!()
    }

    pub fn keys(&self) {
        todo!();
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

    // #[test]
    // fn test_delete() {
    //     todo!()
    // }

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
