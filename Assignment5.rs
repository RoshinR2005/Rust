use std::collections::HashMap;
use std::cmp::Ord;

// Define the SortByKey trait
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement the SortByKey trait for the generic HashMap struct
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut sorted_pairs: Vec<(&K, &V)> = self.iter().collect();
        sorted_pairs.sort_by(|a, b| a.0.cmp(b.0));
        sorted_pairs
    }
}

fn main() {
    // Create a new instance of the generic HashMap struct
    let mut my_map: HashMap<u32, &str> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "apple");
    my_map.insert(1, "banana");
    my_map.insert(2, "cherry");
    my_map.insert(4, "date");

    // Print the original map
    println!("Original map: {:?}", my_map);

    // Sort the elements in the map by their keys using the SortByKey trait
    let sorted_pairs = my_map.sort_by_key();

    // Print the sorted map
    println!("Sorted map: {:?}", sorted_pairs);
}