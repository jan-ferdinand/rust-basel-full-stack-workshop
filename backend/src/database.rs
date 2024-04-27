use std::collections::HashMap;
use std::hash::Hash;

use uuid::Uuid;

pub type DB = InMemoryDatabase<String, ShoppingItem>;

#[derive(Debug, Clone, PartialEq)]
pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

impl ShoppingItem {
    pub fn new(title: String, creator: String) -> Self {
        Self { title, creator }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InMemoryDatabase<K, V>(HashMap<K, V>)
where
    K: Eq + Hash + Clone,
    V: Clone;

impl<K, V> InMemoryDatabase<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn create(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, v)
    }

    fn _read(&self, k: &K) -> Option<&V> {
        self.0.get(k)
    }

    pub fn delete(&mut self, k: &K) -> Option<V> {
        self.0.remove(k)
    }

    pub fn to_vec(&self) -> Vec<(K, V)> {
        let to_owned = |(k, v): (&K, &V)| (k.to_owned(), v.to_owned());
        self.0.iter().map(to_owned).collect()
    }
}

impl Default for DB {
    fn default() -> Self {
        let salt = (
            Uuid::new_v4().to_string(),
            ShoppingItem::new("Salt".to_string(), "Yasin".to_string()),
        );
        let pepper = (
            Uuid::new_v4().to_string(),
            ShoppingItem::new("Pepper".to_string(), "Tim".to_string()),
        );

        Self([salt, pepper].into_iter().collect())
    }
}
