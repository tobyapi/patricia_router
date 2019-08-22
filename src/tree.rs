use crate::node::*;
use crate::result::*;
use crate::utils::*;

/// A [Radix tree](https://en.wikipedia.org/wiki/Radix_tree) implementation.
pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            root: Node::<T>::new("", None, true),
        }
    }

    /// Adds *path* into the Tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_router::Tree;
    ///
    /// let mut tree = Tree::<&str>::new();
    /// tree.add("/abc", "root");
    /// ```
    pub fn add(&mut self, path: impl Into<String>, payload: T) -> () {
        if self.root.placeholder {
            self.root = Node::<T>::new(&path.into(), Some(payload), false);
        } else {
            Tree::<T>::add_internal(&path.into(), Some(payload), &mut self.root);
        }
    }

    fn add_internal(path: &String, payload: Option<T>, node: &mut Node<T>) -> () {
        let mut rest_path_peekable = path.chars().peekable();
        let mut rest_key_peekable = node.key.chars().peekable();
        let mut pos = 0;

        loop {
            let p = rest_path_peekable.peek();
            let k = rest_key_peekable.peek();
            if p.is_none() || k.is_none() || p != k {
                break;
            }
            rest_path_peekable.next();
            rest_key_peekable.next();
            pos += 1;
        }

        let rest_path = rest_path_peekable.collect::<String>();
        let key_size = node.key.bytes().len();
        let path_size = path.bytes().len();

        if pos == 0 || (key_size <= pos && pos < path_size) {
            let new_key = rest_path.as_str();
            let child_op = node
                .children
                .iter_mut()
                .find(|child| same_first_char(new_key, &child.key));

            match child_op {
                Some(mut child) => {
                    Tree::<T>::add_internal(&new_key.to_string(), payload, &mut child)
                }
                None => node.children.push(Node::<T>::new(new_key, payload, false)),
            }
            node.sort_children();
        } else if key_size == pos && pos == path_size {
            if node.payload.is_some() {
                panic!("duplicate error");
            }
            node.payload = payload;
        } else if 0 < pos && pos < key_size {
            let rest_key = rest_key_peekable.collect::<String>();
            let new_key = rest_key.as_str();
            let mut new_node: Node<T> = Node::<T>::new(new_key, None, false);
            new_node.payload = std::mem::replace(&mut node.payload, None);
            new_node.children = std::mem::replace(&mut node.children, vec![]);
            node.set_key(prefix(path, pos));
            node.children.push(new_node);
            if pos < path_size {
                node.children
                    .push(Node::<T>::new(rest_path.as_str(), payload, false));
            } else {
                node.payload = payload;
            }
            node.sort_children();
        }
    }

    /// Returns a `patricia_router::result::Result` after walking the tree looking up for *path*.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_router::Tree;
    ///
    /// let mut tree = Tree::<&str>::new();
    /// tree.add("/about", "about");
    /// let result = tree.find("/about");
    /// ```
    pub fn find<'a>(&'a self, path: impl Into<String>) -> Result<'a, T> {
        let result = Result::<'a, T>::new();
        return Tree::<T>::find_internal(&path.into(), result, &self.root, true);
    }

    fn find_internal<'a>(
        path: &str,
        mut result: Result<'a, T>,
        node: &'a Node<T>,
        first: bool,
    ) -> Result<'a, T> {
        let key_size = node.key.chars().count();
        let path_size = path.chars().count();
        if first && path_size == key_size && path == &node.key && node.payload.is_some() {
            return result.add(node, true);
        }

        let mut path_pos = 0;
        let mut key_pos = 0;
        loop {
            let path_current = path.chars().nth(path_pos);
            let key_current = node.key.chars().nth(key_pos);
            if path_current.is_none() || key_current.is_none() {
                break;
            }
            if key_current != Some('*') && key_current != Some(':') && path_current != key_current {
                break;
            }
            if let Some(k) = key_current {
                if k == '*' {
                    let name = suffix(&node.key, key_pos + 1);
                    let value = suffix(path, path_pos);
                    result.params.insert(name, value);
                    return result.add(node, true);
                } else if k == ':' {
                    let key_size = detect_param_size(&node.key, key_pos);
                    let path_size = detect_param_size(path, path_pos);
                    let name = substring(&node.key, key_pos + 1, key_size);
                    let value = substring(path, path_pos, path_size);
                    result.params.insert(name, value);
                    path_pos += path_size;
                    key_pos += key_size;
                }
            }
            path_pos += 1;
            key_pos += 1;
        }

        let path_next = path.chars().nth(path_pos);
        let key_next = node.key.chars().nth(key_pos);

        if path_next.is_none() && key_next.is_none() && node.payload.is_some() {
            return result.add(node, true);
        }

        if path_next.is_some() {
            if 0 < key_size && has_trailing_slash(path_pos, path_size, path) {
                return result.add(node, true);
            }

            let new_path = suffix(path, path_pos);
            if let Some(child) = node
                .children
                .iter()
                .find(|&child| child.is_named_or_catch_all() || shared_key(&new_path, &child.key))
            {
                result = result.add(node, false);
                return Tree::<T>::find_internal(&new_path, result, &child, false);
            }
            return result;
        }

        if key_next.is_some() {
            if has_trailing_slash(key_pos, key_size, &node.key) {
                return result.add(node, true);
            }

            if node.has_catch_all(key_pos, key_size) {
                if key_next != Some('*') {
                    key_pos += 1;
                }
                let name = suffix(&node.key, key_pos + 1);
                result.params.insert(name, String::new());
                return result.add(node, true);
            }
        }
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn single_node() {
        let mut router = Tree::<&str>::new();
        router.add("/abc", "root");
        assert_eq!(router.root.payload, Some("root"));
    }

    #[test]
    fn shared_root() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/a", "a");
        router.add("/bc", "bc");
        /*
            /    (:root)
            +-bc (:bc)
            \-a  (:a)
        */
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[0].key, "bc");
        assert_eq!(router.root.children[0].payload, Some("bc"));
        assert_eq!(router.root.children[1].key, "a");
        assert_eq!(router.root.children[1].payload, Some("a"));
    }

    #[test]
    fn shared_parent() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/abc", "abc");
        router.add("/axyz", "axyz");
        /*
            /       (:root)
            +-a
              +-xyz (:axyz)
              \-bc  (:abc)
        */
        assert_eq!(router.root.children.len(), 1);
        assert_eq!(router.root.children[0].key, "a");
        assert_eq!(router.root.children[0].children.len(), 2);
        assert_eq!(router.root.children[0].children[0].key, "xyz");
        assert_eq!(router.root.children[0].children[1].key, "bc");
    }

    #[test]
    fn multiple_parent_nodes() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/admin/users", "users");
        router.add("/admin/products", "products");
        router.add("/blog/tags", "tags");
        router.add("/blog/articles", "articles");
        /*
            /                 (:root)
            +-admin/
            |      +-products (:products)
            |      \-users    (:users)
            |
            +-blog/
                  +-articles  (:articles)
                  \-tags      (:tags)
        */
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[0].key, "admin/");
        assert_eq!(router.root.children[0].payload, None);
        assert_eq!(router.root.children[0].children[0].key, "products");
        assert_eq!(router.root.children[0].children[1].key, "users");

        assert_eq!(router.root.children[1].key, "blog/");
        assert_eq!(router.root.children[1].payload, None);
        assert_eq!(router.root.children[1].children[0].key, "articles");
        assert_eq!(
            router.root.children[1].children[0].payload,
            Some("articles")
        );
        assert_eq!(router.root.children[1].children[1].key, "tags");
        assert_eq!(router.root.children[1].children[1].payload, Some("tags"));
    }

    #[test]
    fn multiple_nodes_with_mixed_parents() {
        let mut router = Tree::<&str>::new();
        router.add("/authorizations", "authorizations");
        router.add("/authorizations/:id", "authorization");
        router.add("/applications", "applications");
        router.add("/events", "events");
        /*
            /
            +-events                (:events)
             +-a
               +-uthorizations      (:authorizations)
               |             \-/:id (:authorization)
               \-pplications        (:applications)
        */
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[1].key, "a");
        assert_eq!(router.root.children[1].children.len(), 2);
        assert_eq!(
            router.root.children[1].children[0].payload,
            Some("authorizations")
        );
        assert_eq!(
            router.root.children[1].children[1].payload,
            Some("applications")
        );
    }

    #[test]
    fn insertion_of_mixed_routes_out_of_order() {
        let mut router = Tree::<&str>::new();
        router.add("/user/repos", "my_repos");
        router.add("/users/:user/repos", "user_repos");
        router.add("/users/:user", ":user");
        router.add("/user", "me");
        /*
            /user                (:me)
                +-/repos         (:my_repos)
                \-s/:user        (:user)
                        \-/repos (:user_repos)
        */
        assert_eq!(router.root.key, "/user");
        assert_eq!(router.root.payload, Some("me"));
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[0].key, "/repos");
        assert_eq!(router.root.children[1].key, "s/:user");
        assert_eq!(router.root.children[1].payload, Some(":user"));
        assert_eq!(router.root.children[1].children[0].key, "/repos");
    }

    #[test]
    fn dealing_with_unicode1() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/かきく", "kakiku");
        router.add("/あいうえお", "aiueo");
        /*
            /            (:root)
            +-あいうえお    (:aiueo)
            \-かきく       (:kakiku)
        */
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[0].key, "あいうえお");
        assert_eq!(router.root.children[1].key, "かきく");
    }

    #[test]
    fn dealing_with_unicode2() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/あいう", "aiu");
        router.add("/あいかきくけこ", "aikakikukeko");
        /*
            /               (:root)
            \-あいう          (:aiu)
                \-かきくけこ   (:kakikukeko)
        */
        assert_eq!(router.root.children.len(), 1);
        assert_eq!(router.root.children[0].key, "あい");
        assert_eq!(router.root.children[0].children.len(), 2);
        assert_eq!(router.root.children[0].children[0].key, "かきくけこ");
        assert_eq!(router.root.children[0].children[1].key, "う");
    }

    #[test]
    fn dealing_with_catch_all_and_named_parameters() {
        let mut router = Tree::<&str>::new();
        router.add("/", "root");
        router.add("/*filepath", "all");
        router.add("/products", "products");
        router.add("/products/:id", "product");
        router.add("/products/:id/edit", "edit");
        router.add("/products/featured", "featured");
        /*
            /                      (:all)
            +-products             (:products)
            |        \-/
            |          +-featured  (:featured)
            |          \-:id       (:product)
            |              \-/edit (:edit)
            \-*filepath            (:all)
        */
        assert_eq!(router.root.children.len(), 2);
        assert_eq!(router.root.children[0].key, "products");
        assert_eq!(router.root.children[0].children[0].key, "/");

        let nodes = &router.root.children[0].children[0].children;
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].key, "featured");
        assert_eq!(nodes[1].key, ":id");
        assert_eq!(nodes[1].children[0].key, "/edit");

        assert_eq!(router.root.children[1].key, "*filepath");
    }
}
