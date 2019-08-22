use std::collections::HashMap;

use crate::node::Node;

/// A Result is the comulative output of walking our [Radix tree](https://en.wikipedia.org/wiki/Radix_tree)
#[derive(Debug, PartialEq, Eq)]
pub struct Result<'a, T> {
    key: Option<String>,
    nodes: Vec<&'a Node<T>>,
    pub(crate) params: HashMap<String, String>,
    pub payload: &'a Option<T>,
}

impl<'a, T> Result<'a, T> {
    #[doc(hidden)]
    pub(crate) fn new() -> Self {
        Self {
            key: None,
            nodes: Vec::<&'a Node<T>>::new(),
            params: HashMap::new(),
            payload: &None,
        }
    }

    #[doc(hidden)]
    pub(crate) fn add(mut self, node: &'a Node<T>, payload: bool) -> Self {
        self.nodes.push(node);
        if payload && node.payload.is_some() {
            self.payload = &node.payload;
        }
        self
    }

    /// Returns a String built based on the nodes used in the result.
    pub fn key(&mut self) -> String {
        match &self.key {
            Some(k) => k.to_string(),
            None => {
                let result = self.compute_key();
                self.key = Some(result.clone());
                result
            }
        }
    }

    fn compute_key(&self) -> String {
        self.nodes
            .iter()
            .fold(String::new(), |acc, &node| acc + &node.key)
    }

    /// Returns named or catch-all parameter in the result.
    pub fn params(&'a self, index: impl Into<String>) -> &'a String {
        &self.params[&index.into()]
    }
}

#[cfg(test)]
mod test {
    use super::Result;
    use crate::node::Node;

    #[test]
    fn simple() {
        let node = Node::<&str>::new("/", Some("root"), true);
        let mut result = Result::<&str>::new();
        assert_eq!(result.payload, &None);
        result = result.add(&node, true);
        assert_eq!(result.key(), "/".to_string());
        assert_eq!(result.payload, &Some("root"));
    }

    #[test]
    fn multiple_node() {
        let node1 = Node::<&str>::new("/", Some("root"), true);
        let node2 = Node::<&str>::new("about", Some("about"), true);
        let mut result = Result::<&str>::new();
        result = result.add(&node1, true);
        result = result.add(&node2, true);
        assert_eq!(result.key(), "/about".to_string());
    }

    #[test]
    fn not_assign_payload() {
        let node = Node::<&str>::new("/", Some("root"), true);
        let mut result = Result::<&str>::new();
        assert_eq!(result.payload, &None);
        result = result.add(&node, false);
        assert_eq!(result.payload, &None);
    }
}
