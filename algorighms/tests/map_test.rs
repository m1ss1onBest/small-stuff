#[cfg(test)]
mod map_tests {
    use algorighms::unordered::Map;

    use super::*;

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
}