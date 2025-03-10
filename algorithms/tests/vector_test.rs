
#[cfg(test)]
mod tests {
    use std::{str::RSplitTerminator, time::Instant};

    use algorighms::collections::vector::Vector;

    use super::*;

    #[test]
    fn feature() {
        let runtime_alloc = Instant::now();
        let mut vec = Vector::new();
        for i in 0..100000 {
            vec.push(i);
        }
        let runtime_alloc = runtime_alloc.elapsed();

        let runtime_print = Instant::now();
        vec.iter().filter(|&x| x % 17 == 0).for_each(|x| println!("{x}"));

        println!("length: {}", vec.len());
        println!("capacity: {}", vec.capacity());
        println!("runtime alloc: {:?}", runtime_alloc);
        println!("runtime print: {:?}", runtime_print.elapsed());

        for i in 0..100000 {
            let v = vec.pop();
            // println!("{v:?}");
        }
    }
}
