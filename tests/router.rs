use patricia_router::Router;

#[test]
fn single_node() {
    let mut router = Router::<&str>::new();
    router.add("/about", "about");
    assert_eq!(router.find("/about").key(), "/about");
    assert_eq!(router.find("/products").key(), "");
}

#[test]
fn key_and_path_matches() {
    let mut router = Router::<&str>::new();
    router.add("/about", "about");
    let mut result = router.find("/about");
    assert_eq!(result.key(), "/about");
    assert_eq!(result.payload, &Some("about"));
}

#[test]
fn nodes_with_shared_parent() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/abc", "abc");
    router.add("/axyz", "axyz");

    let mut result = router.find("/abc");
    assert_eq!(result.key(), "/abc");
    assert_eq!(result.payload, &Some("abc"));
}

#[test]
fn matching_path_across_separator() {
    let mut router = Router::<&str>::new();
    router.add("/products", "products");
    router.add("/product/new", "product_new");

    let mut result = router.find("/products");
    assert_eq!(result.key(), "/products");
    assert_eq!(result.payload, &Some("products"));
}

#[test]
fn matching_path_across_parents() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/admin/users", "users");
    router.add("/admin/products", "products");
    router.add("/blog/tags", "tags");
    router.add("/blog/articles", "articles");

    let mut result = router.find("/blog/tags/");
    assert_eq!(result.key(), "/blog/tags");
    assert_eq!(result.payload, &Some("tags"));
}

#[test]
fn unicode_nodes_with_shared_parent() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/あいう", "aiu");
    router.add("/あいかきくけこ", "aikakikukeko");

    let mut result = router.find("/あいかきくけこ/");
    assert_eq!(result.key(), "/あいかきくけこ");
}

#[test]
fn matching_path() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/about", "about");

    let result = router.find("/*filepath");
    assert_eq!(result.payload, &Some("all"));
}

#[test]
fn catch_all_in_parameters() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/about", "about");

    let result = router.find("/src/file.png");
    assert_eq!(result.params("filepath"), "src/file.png");
}

#[test]
fn returns_optional_catch_all_after_slash() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/search/*extra", "extra");

    let result = router.find("/search");
    assert_eq!(result.params("extra"), "");
}

#[test]
fn returns_optional_catch_all_by_globbing() {
    let mut router = Router::<&str>::new();
    router.add("/members*trailing", "members_catch_all");

    let result = router.find("/members");
    assert_eq!(result.params("trailing"), "");
}

#[test]
fn does_not_when_catch_all_is_not_full_match() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/serach/public/*query", "search");

    let mut result = router.find("/search");
    assert_eq!(result.key(), "");
}

#[test]
fn does_not_when_path_search_has_been_exhausted() {
    let mut router = Router::<&str>::new();
    router.add("/members/*training", "members_catch_all");

    let mut result = router.find("/members2");
    assert_eq!(result.key(), "");
}

#[test]
fn does_prefer_specific_path_over_catch_all_if_both_are_present() {
    let mut router = Router::<&str>::new();
    router.add("/members", "members");
    router.add("/members/*training", "members_catch_all");

    let mut result = router.find("/members");
    assert_eq!(result.key(), "/members");
}

#[test]
fn does_prefer_catch_all_over_specific_key_with_partially_shared_key() {
    let mut router = Router::<&str>::new();
    router.add("/orders/*anything", "orders_catch_all");
    router.add("/orders/closed", "closed_orders");

    let mut result = router.find("/orders/cancelled");
    assert_eq!(result.key(), "/orders/*anything");
    assert_eq!(result.params("anything"), "cancelled");
}

#[test]
fn dealing_with_named_parameters() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");

    let mut result = router.find("/products/10");
    assert_eq!(result.key(), "/products/:id");
    assert_eq!(result.payload, &Some("product"));
}

#[test]
fn does_not_partial_matchin_path() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/products", "products");
    router.add("/products/:id/edit", "edit");

    let result = router.find("/products/10");
    assert_eq!(result.payload, &None);
}

#[test]
fn returns_named_parameters_in_result() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");

    let result = router.find("/products/10/edit");
    assert_eq!(result.params("id"), "10");
}

#[test]
fn returns_unicode_values_in_parameters() {
    let mut router = Router::<&str>::new();
    router.add("/one/:あいう", "one");
    let mut result = router.find("/one/10");
    assert_eq!(result.key(), "/one/:あいう");
    assert_eq!(result.params("あいう"), "10");
}

#[test]
fn does_prefer_specific_path_over_named_parameter_one_if_both_are_present() {
    let mut router = Router::<&str>::new();
    router.add("/tag-edit/:tag", "root");
    router.add("/tag-edit2", "products");

    let mut result = router.find("/tag-edit2");
    assert_eq!(result.key(), "/tag-edit2");
}

#[test]
fn does_prefer_named_parameter_over_specific_key_with_partially_shared_key() {
    let mut router = Router::<&str>::new();
    router.add("/orders/:id", "specific_order");
    router.add("/orders/closed", "closed_orders");

    let mut result = router.find("/orders/10");
    assert_eq!(result.key(), "/orders/:id");
    assert_eq!(result.params("id"), "10");
}

#[test]
fn dealing_with_multiple_named_parameters() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/:section/:page", "static_page");

    let mut result = router.find("/about/shipping");
    assert_eq!(result.params("section"), "about");
    assert_eq!(result.params("page"), "shipping");

    result = router.find("/:section/:page");
    assert_eq!(result.key(), "/:section/:page");
    assert_eq!(result.payload, &Some("static_page"));
}

#[test]
fn dealing_with_both_catch_all_and_named_parameters() {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");

    let mut result = router.find("/products/1000");
    assert_eq!(result.key(), "/products/:id");
    assert_eq!(result.payload, &Some("product"));

    result = router.find("/admin/articles");
    assert_eq!(result.key(), "/*filepath");
    assert_eq!(result.params("filepath"), "admin/articles");

    result = router.find("/products/featured");
    assert_eq!(result.key(), "/products/featured");
    assert_eq!(result.payload, &Some("featured"));
}

#[test]
fn dealing_with_named_parameters_and_shared_key() {
    let mut router = Router::<&str>::new();
    router.add("/one/:id", "one");
    router.add("/one-longer/:id", "two");
    let mut result = router.find("/one-longer/10");
    assert_eq!(result.key(), "/one-longer/:id");
    assert_eq!(result.params("id"), "10");
}
