pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mul(lhs: usize, rhs: usize) -> usize {
    lhs.checked_mul(rhs).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
