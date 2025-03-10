#[cfg(test)]
mod tests {
    use std::time::Instant;
    use rand::Rng;
    use algorighms::collections::Vector;

    use super::*;

    #[test]
    fn test_new() {
        let mut vec: Vector<i32> = Vector::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);

        vec.push(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_with_capacity() {
        let mut vec: Vector<i32> = Vector::with_capacity(13);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 13);

        for i in 0..14 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 14);
        assert_eq!(vec.capacity(), 26);
    }

    #[test]
    fn test_back() {
        let runtime = Instant::now();
        let mut vec = Vector::new();
        for i in 0..100000 {
            vec.push(i);
        }
        println!("len: {}, capacity: {}", vec.len(), vec.capacity());

        while let Some(_) = vec.pop() { 
            // some code idk
        }

        println!("len: {}, capacity: {}", vec.len(), vec.capacity());
        println!("runtime: {:?}", runtime.elapsed());
    }

    #[test]
    fn test_front() {
        let runtime = Instant::now();
        let mut vec = Vector::new();
        for i in 0..100000 {
            vec.push_front(i);
        }
        println!("len: {}, capacity: {}", vec.len(), vec.capacity());

        while let Some(_) = vec.pop_front() { 
            // some code idk
        }

        println!("len: {}, capacity: {}", vec.len(), vec.capacity());
        println!("runtime: {:?}", runtime.elapsed());
    }

    #[test]
    fn test_chaos() {
        type V = Vector<i32>;
        let funcs = [
            |v: &mut V| v.push(0),
            |v: &mut V| v.push_front(0),
            |v: &mut V| _ = v.pop(),
            |v: &mut V| _ = v.pop_front(),
        ];

        let mut vec = Vector::new();
        let mut rng = rand::rng();
        for _ in 0..100000 {
            funcs[rng.random_range(0..4)](&mut vec);
        }

        println!("length: {}, capacity: {}", vec.len(), vec.capacity());
        println!("the vector has been reallocated {} times", (vec.capacity() as f32).log2())
    }

    #[test]
    fn test_index() {
        let mut vec = Vector::new();
        for i in 0..16 {
            vec.push(i);
        }
        for i in 0..16 {
            vec[i] *= 2;
        }

        assert_eq!(vec[5], 10);
    }
}
