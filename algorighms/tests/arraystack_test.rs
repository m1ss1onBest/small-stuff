#[cfg(test)]
mod tests {
    use algorighms::collections::ArrayStack;

    use super::*;

    #[test]
    fn test_arraystack() {
        let mut stack = ArrayStack::new();
        stack.push(5);
    }
}