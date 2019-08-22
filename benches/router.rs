#![feature(test)]
extern crate test;

use test::Bencher;

use patricia_router::Router;

#[bench]
fn router_add(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut router = Router::<&str>::new();
        router.add("/aaa/bbb", "payload")
    });
}

#[bench]
fn router_find_simple(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/aaa/bbb", "payload");
    bencher.iter(|| router.find("/aaa/bbb"));
}

#[bench]
fn router_find_normal(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");

    router.add("/aaa/bbb", "payload");
    bencher.iter(|| router.find("/aaa/bbb"));
}

#[bench]
fn router_find_placeholder(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");
    bencher.iter(|| router.find("/products/10"));
}

#[bench]
fn router_find_all(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");
    bencher.iter(|| router.find("/src_file"));
}

#[bench]
fn router_find_simple_with_placeholder(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");
    bencher.iter(|| router.find("/products/featured"));
}

#[bench]
fn router_find_long_path(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");
    router.add(
        "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z",
        "long",
    );

    bencher.iter(|| router.find("/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z"));
}

#[bench]
fn router_find_long_string(bencher: &mut Bencher) {
    let mut router = Router::<&str>::new();
    router.add("/", "root");
    router.add("/*filepath", "all");
    router.add("/products", "products");
    router.add("/products/:id", "product");
    router.add("/products/:id/edit", "edit");
    router.add("/products/featured", "featured");
    router.add(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "long",
    );

    bencher.iter(|| router.find("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}
