fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);

        // How this was solved:
        // let y = &mut x;
        // let z = &mut x;
        // Is not possible because there can only be 1 'mutable borrows' of something at 1 time.
        // Solution: Mutably borrow, then use once. Then you can mutably borrow again.
    }
}
