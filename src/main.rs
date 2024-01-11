use std::collections::hash_map::DefaultHasher;
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

// TODO: handle collision?

// TODO: handle resize

type Buckets = Vec<Option<KV>>;

#[derive(Clone, Debug)]
struct KV {
    key: String,
    value: u64, // TODO: generic
}

struct HashTable {
    buckets: Buckets,
    size: usize,
}

impl HashTable {
    pub fn new() -> Self {
        let buckets: Buckets = vec![None; 100]; // NOTE: init size?

        HashTable { buckets, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, key: String, value: u64) {
        let kv = KV {
            key: key.clone(),
            value,
        };

        let index = self.create_index(key);

        self.buckets[index] = Some(kv);
        self.size += 1;
    }

    pub fn get(&self, key: String) -> Option<u64> {
        let index = self.create_index(key);
        let kv = self.buckets[index].clone();

        // TODO: check if keys are equal

        // self.buckets[index].clone().map(|kv| kv.value)

        todo!()
    }

    pub fn delete(&mut self, key: String) {
        todo!()
    }

    fn create_index(&self, key: String) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash = s.finish();

        // Modulo arithmetic -> Uniform Distribution + Efficiency
        (hash % (self.buckets.len() as u64)) as usize
    }
}

fn main() {
    let hash_table = HashTable::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hash_table = HashTable::new();
        assert_eq!(hash_table.size(), 0);
    }

    #[test]
    fn test_insert() {
        let mut hash_table = HashTable::new();

        hash_table.insert("key1".to_string(), 1);
        hash_table.insert("key2".to_string(), 2);
        hash_table.insert("key3".to_string(), 3);

        assert_eq!(hash_table.size(), 3);

        assert_eq!(hash_table.get("key1".to_string()), Some(1));
        assert_eq!(hash_table.get("key3".to_string()), Some(3));
    }

    #[test]
    fn test_delete() {
        todo!()
    }
}
