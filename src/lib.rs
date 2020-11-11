mod errors;

fn add() -> u32 {
    return 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(add(), 1);
    }
}
