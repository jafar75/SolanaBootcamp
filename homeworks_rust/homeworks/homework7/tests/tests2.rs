// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


fn someNumber() -> i64 {
    return 123;
}

#[cfg(test)]
mod tests {
    use someNumber;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(someNumber(), 123);
    }
}
