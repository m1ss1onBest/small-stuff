#[cfg(test)]
mod map_tests {
    use algorighms::unordered::Map;

    fn get_map() -> Map<i32, &'static str> {
        let mut map = Map::new();
 
        map.insert(2, "bob");
        map.insert(1, "hello");
        map.insert(3, "hello");
        map.insert(1, "tim");
        map
    }

    #[test]
    fn map_insert_test() {
        let map = get_map();
        println!("{:#?}", map);
    }

    #[test]
    fn map_get_test() {
        let mut map = get_map();
        if let Some(x) = map.get_mut(2) {
            *x = "coca cola espuma"
        }

        println!("map by index 1 is {:?}", map.get(1));
        println!("map by index 2 is {:?}", map.get(2));
    }

    #[test]
    fn map_entry_or_insert_test() {
        let mut map = Map::new();
        map.insert("john", 1);
        map.insert("kate", 2);
        map.insert("vovan", 3);
        
        *map.entry("kate").or_insert(11) += 1000;
        *map.entry("ivan").or_insert(11) += 1000;
    }

    #[test]
    fn map_index_test() {
        let map = get_map();
        assert_eq!(map[1], "tim");
    }
}