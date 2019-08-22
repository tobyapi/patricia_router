pub(crate) fn detect_param_size(key: &str, old_pos: usize) -> usize {
    let rest_key = key.chars().skip(old_pos).collect::<String>();

    if let Some(pos) = rest_key.chars().position(|ch| ch == '/') {
        return old_pos + pos;
    }
    old_pos + rest_key.len()
}

pub(crate) fn same_first_char(a: &str, b: &str) -> bool {
    let a_first = a.chars().next();
    let b_first = b.chars().next();
    if a_first == Some(':') && b_first == Some(':') && !same_key(a, b) {
        panic!("shared key error")
    }
    a_first == b_first
}

fn same_key(path: &str, key: &str) -> bool {
    let mut it = path
        .chars()
        .zip(key.chars())
        .skip_while(|&(p, k)| (p != '/') && (k != '/') && (p == k));

    match it.next() {
        Some((p, _k)) => p == '/',
        None => path.len() < key.len(),
    }
}

fn check_markers(ch: Option<char>) -> bool {
    ch == Some('/') || ch == Some(':') || ch == Some('*')
}

pub(crate) fn shared_key(path: &str, key: &str) -> bool {
    let key_first = key.chars().next();
    if path.chars().next() != key_first && check_markers(key_first) {
        return false;
    }

    let mut rest_path = path.chars();
    let mut rest_key = key.chars();

    loop {
        let p = rest_path.next();
        let k = rest_key.next();
        if p.is_none() || k.is_none() || check_markers(p) || check_markers(k) {
            return k.is_none() || check_markers(k);
        }
        if p != k {
            return false;
        }
    }
}

pub(crate) fn substring(target: &str, begin: usize, end: usize) -> String {
    target
        .chars()
        .skip(begin)
        .take(end - begin)
        .collect::<String>()
}

pub(crate) fn prefix(target: &str, end: usize) -> String {
    target.chars().take(end).collect::<String>()
}

pub(crate) fn suffix(target: &str, begin: usize) -> String {
    target.chars().skip(begin).collect::<String>()
}

pub(crate) fn has_trailing_slash(end: usize, size: usize, path: &str) -> bool {
    end + 1 == size && path.chars().nth(end) == Some('/')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_key() {
        assert!(!same_key("foo", "bar"));
        assert!(same_key("foo/bar", "foo/baz"));
        assert!(!same_key("zipcode", "zip"));
        assert!(same_key("zip", "zipcode"));
        assert!(!same_key("s", "/new"));
        assert!(same_key("foo/bar", "fooa/baz"));
        assert!(!same_key("fooa/bar", "foo/baz"));
    }

    #[test]
    fn test_shared_key() {
        assert!(!shared_key("foo", "bar"));
        assert!(shared_key("foo/bar", "foo/baz"));
        assert!(shared_key("zipcode", "zip"));
        assert!(!shared_key("zip", "zipcode"));
        assert!(!shared_key("s", "/new"));
        assert!(!shared_key("foo/bar", "fooa/baz"));
        assert!(shared_key("fooa/bar", "foo/baz"));
        assert!(shared_key("search", "search/*extra"));
    }

    #[test]
    fn test_substring() {
        assert_eq!(substring("abcde", 1, 3), "bc");
        assert_eq!(substring("あいうえお", 2, 4), "うえ");
    }

    #[test]
    fn test_prefix() {
        assert_eq!(prefix("abcde", 3), "abc");
        assert_eq!(prefix("あいうえお", 4), "あいうえ");
    }

    #[test]
    fn test_suffix() {
        assert_eq!(suffix("abcde", 1), "bcde");
        assert_eq!(suffix("あいうえお", 2), "うえお");
    }
}
