/// Hello world test unit
///
/// # Examples
///
/// ```
/// # use stack_deploy::unit;
/// assert_eq!(unit(), ());
/// ```
pub fn unit() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(unit(), ());
    }
}
