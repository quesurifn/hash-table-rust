use std::rc::Rc;

pub struct FnvHasher {
    prime: u64,
    offset: u64,
}
impl FnvHasher {
    pub fn new() -> Self {
        Self {
            prime: 0x100000001b3,
            offset: 0xcbf29ce484222325,
        }
    }

    pub fn hash(&self, value: &[u8], array_size: i64) -> u64 {
        let mut final_hash = self.offset;
        for byte in value.iter() {
            final_hash = final_hash ^ (*byte as u64);
            final_hash = final_hash.wrapping_mul(self.prime);
        }

        final_hash % (array_size as u64)
    }
}

#[derive(Debug, Clone)]
pub struct KeyValue<T> {
    key: String,
    value: T,
}
impl<T> KeyValue<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone)]
pub struct  HashTable<T> {
    max_size: i64,
    array_length: i64,
    array: Vec<Vec<Rc<KeyValue<T>>>>,
}

impl<T> HashTable<T> {
    pub fn new() -> Self {
        Self {
            max_size: 1000,
            array_length: 0,
            array: vec![Vec::new(); 1000]
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        let hasher = FnvHasher::new();
        let hash = hasher.hash(key.as_bytes(), self.max_size);
        let new_value = KeyValue::new(key, value);
        self.array[hash as usize].push(Rc::new(new_value));
        self.array_length += 1;
    }


    pub fn get(&self, key: String) -> Option<&T> {
        let hasher = FnvHasher::new();
        let hash = hasher.hash(key.as_bytes(), self.max_size);
        for item in self.array[hash as usize].iter() {
            if item.key == key {
                return Some(&item.value)
            }
        }
        None
    }

    pub fn remove(&mut self, key: String) {
        let hasher = FnvHasher::new();
        let hash = hasher.hash(key.as_bytes(), self.max_size);

        for (index, item) in self.array[hash as usize].iter().enumerate() {
            if item.key == key {
                self.array[hash as usize].remove(index);
                return;
            }
        }
        self.array_length -= 1;
    }
}




fn main() {
    let hasher = FnvHasher::new();
    let hash = hasher.hash(b"hello", 100);
    println!("Hash: {}", hash);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    use crate::HashTable;

    #[test]
    fn test_add() {
        let mut table = HashTable::<String>::new();
        table.insert("hello".to_string(), "world".to_string());
        table.insert("hello2".to_string(), "world2".to_string());
        table.insert("hello3".to_string(), "world3".to_string());


        assert_eq!(table.get("hello".to_string()).unwrap(), "world");
        assert_eq!(table.get("hello2".to_string()).unwrap(), "world2");
        assert_eq!(table.get("hello3".to_string()).unwrap(), "world3");
    }
}


