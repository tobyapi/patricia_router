use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node<T> {
    pub key: String,
    pub payload: Option<T>,
    pub(crate) placeholder: bool,
    pub(crate) children: Vec<Node<T>>,
    kind: Kind,
    priority: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Kind {
    Normal,
    Named,
    Glob,
}

impl<T> Node<T> {
    pub(crate) fn new(k: impl Into<String>, payload: Option<T>, placeholder: bool) -> Self {
        let key = k.into();
        let (priority, kind) = Node::<T>::compute_priority(&key);
        Self {
            key,
            placeholder,
            children: Vec::<Node<T>>::new(),
            payload,
            kind,
            priority,
        }
    }

    fn compute_priority(key: &String) -> (i32, Kind) {
        for (i, current_char) in key.chars().enumerate() {
            if current_char == '*' {
                return (i as i32, Kind::Glob);
            } else if current_char == ':' {
                return (i as i32, Kind::Named);
            }
        }
        (key.len() as i32, Kind::Normal)
    }

    pub(crate) fn set_key(&mut self, value: String) -> () {
        self.key = value;
        let (p, k) = Node::<T>::compute_priority(&self.key);
        self.priority = p;
        self.kind = k;
    }

    pub(crate) fn sort_children(&mut self) -> () {
        self.children.sort_by(|a, b| a.cmp(b))
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.kind.cmp(&other.kind);
        if result != Ordering::Equal {
            return result;
        }
        other.priority.cmp(&self.priority)
    }

    pub(crate) fn has_catch_all(&self, pos: usize, size: usize) -> bool {
        let mut a = self.key.chars();
        let current = a.nth(pos);
        let next = a.next();
        pos < size && ((current == Some('/') && next == Some('*')) || current == Some('*'))
    }

    pub(crate) fn is_named_or_catch_all(&self) -> bool {
        let first_char = self.key.chars().next();
        first_char == Some('*') || first_char == Some(':')
    }
}

#[cfg(test)]
mod test {
    use super::{Kind, Node};

    #[test]
    fn key() {
        let mut node = Node::<()>::new("abc", None, true);
        assert_eq!(node.key, "abc");

        node.key = "xyz".to_string();
        assert_eq!(node.key, "xyz");
    }

    #[test]
    fn kind() {
        let mut node = Node::<()>::new("a", None, true);
        assert_eq!(node.kind, Kind::Normal);

        node = Node::<()>::new(":query", None, true);
        assert_eq!(node.kind, Kind::Named);

        node = Node::<()>::new("*filepath", None, true);
        assert_eq!(node.kind, Kind::Glob);
    }

    #[test]
    fn payload() {
        let node = Node::<&str>::new("abc", Some("payload"), true);
        assert_eq!(node.payload, Some("payload"));
    }

    #[test]
    fn priority() {
        let mut node = Node::<()>::new("a", None, true);
        assert_eq!(node.priority, 1);

        node = Node::<()>::new("abc", None, true);
        assert_eq!(node.priority, 3);
    }

    #[test]
    fn priority_named() {
        let mut node = Node::<()>::new("/posts/:id", None, true);
        assert_eq!(node.priority, 7);

        node = Node::<()>::new("/u/:username", None, true);
        assert_eq!(node.priority, 3);
    }

    #[test]
    fn priority_glob() {
        let mut node = Node::<()>::new("/search/*query", None, true);
        assert_eq!(node.priority, 8);

        node = Node::<()>::new("/*anything", None, true);
        assert_eq!(node.priority, 1);
    }

    #[test]
    fn sort() {
        let mut root = Node::<i32>::new("/", None, true);
        let node1 = Node::<i32>::new("a", None, true);
        let node2 = Node::<i32>::new("bc", None, true);
        let node3 = Node::<i32>::new("def", None, true);
        root.children = vec![node1, node2, node3];
        root.sort_children();

        assert_eq!(root.children[0].key, "def");
        assert_eq!(root.children[1].key, "bc");
        assert_eq!(root.children[2].key, "a");
    }

    #[test]
    fn sort_named_and_glob() {
        let mut root = Node::<i32>::new("/", None, true);
        let node1 = Node::<i32>::new("*filepath", None, true);
        let node2 = Node::<i32>::new("abc", None, true);
        let node3 = Node::<i32>::new(":query", None, true);
        root.children = vec![node1, node2, node3];
        root.sort_children();

        assert_eq!(root.children[0].key, "abc");
        assert_eq!(root.children[1].key, ":query");
        assert_eq!(root.children[2].key, "*filepath");
    }
}
