
pub fn transform(name: &str, prefix: &str) -> String {
    (if name.starts_with(prefix) {
        name.replacen(prefix, "", 1)
    } else {
        String::from(name)
    }).to_uppercase().replace("-", "_").replace("/", "_")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_upper() {
        assert_eq!(transform("/something", "/"), "SOMETHING");
    }

    #[test]
    fn to_snake() {
        assert_eq!(transform("/a-value", "/"), "A_VALUE");
    }

    #[test]
    fn flattens() {
        assert_eq!(transform("/a/path/to/a/value", "/"), "A_PATH_TO_A_VALUE");
    }

}
