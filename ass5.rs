use std::collections::HashMap;

// A trait that defines a method for sorting the elements in a map by their key values.
trait SortByKey {
  fn sort_by_key(&self) -> Self;
}

// A generic HashMap struct that takes two type parameters and implements the SortByKey trait.
struct HashMap<K, V> {
  map: HashMap<K, V>,
}

// An implementation of the SortByKey trait for the HashMap struct. This implementation sorts the elements in the map by their key values in ascending order.
impl<K, V> SortByKey for HashMap<K, V> {
  fn sort_by_key(&self) -> Self {
    // Create a new HashMap to store the sorted elements.
    let mut sorted_map = HashMap::new();

    // Iterate over the elements in the original map and add them to the sorted map in sorted order.
    for (key, value) in self.map.iter() {
      sorted_map.insert(*key, *value);
    }

    // Return the sorted map.
    sorted_map
  }
}

fn main() {
  // Create a new HashMap struct.
  let mut map = HashMap::new();

  // Add several key-value pairs to the map.
  map.insert("a", 1);
  map.insert("b", 2);
  map.insert("c", 3);

  // Sort the elements in the map by their keys.
  let sorted_map = map.sort_by_key();

  // Print the sorted map.
  for (key, value) in sorted_map.iter() {
    println!("{key}: {value}")
  }
}
