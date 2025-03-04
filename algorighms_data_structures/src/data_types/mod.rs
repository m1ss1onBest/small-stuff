pub mod vector;

#[cfg(test)]
mod vector_tests {
    use std::time::Instant;

    use super::{vector::Vector, *};

    #[test]
    fn test_push() {
        let mut a = Vector::new();
        a.push(1);
        a.push(1);
        a.push(1);

        println!("vec: {:?}", a);
    }

    #[test]
    fn foo() {
        let mut vec = Vector::new();

        let timer = Instant::now();
        for i in 0..100000 {
            vec.push(i);
        }

        // assert_eq!(vec.len(), 1000);
        println!("capacity: {}", vec.capacity());
        // println!("{:?}", vec);
        println!("runtime: {:?}", timer.elapsed());
    }
}