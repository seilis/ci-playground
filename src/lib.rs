
/// This function adds
///
/// # Examples
/// ```
/// use ci_testing::add;
/// assert_eq!(add(2,3), 4) ;
/// ```
pub fn add(x: u32, y: u32) -> u32 {
   x + y 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn test_add2() {
        assert_eq!(add(2,2), 4, "I messed up adding");
    }
}
