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

    #[test]
    fn map_get_test() {
        let mut map = get_map();
        if let Some(x) = map.get_mut(2) {
            *x = "coca cola espuma"
        }

        println!("map by index 1 is {:?}", map.get(1));
        println!("map by index 2 is {:?}", map.get(2));
    }

}