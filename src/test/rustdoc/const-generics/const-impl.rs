#![feature(const_generics)]

#![crate_name = "foo"]

#[derive(PartialEq, Eq)]
pub enum Order {
    Sorted,
    Unsorted,
}

// @has foo/struct.VSet.html '//pre[@class="rust struct"]' 'pub struct VSet<T, const ORDER: Order>'
// @has foo/struct.VSet.html '//div[@id="impl-Send"]/code' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
// @has foo/struct.VSet.html '//div[@id="impl-Sync"]/code' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
pub struct VSet<T, const ORDER: Order> {
    inner: Vec<T>,
}

// @has foo/struct.VSet.html '//div[@id="impl"]/code' 'impl<T> VSet<T, {Order::Sorted}>'
impl <T> VSet<T, {Order::Sorted}> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

// @has foo/struct.VSet.html '//div[@id="impl-1"]/code' 'impl<T> VSet<T, {Order::Unsorted}>'
impl <T> VSet<T, {Order::Unsorted}> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

pub struct Escape<const S: &'static str>;

// @has foo/struct.Escape.html '//div[@id="impl"]/code' 'impl Escape<{ r#"<script>alert("Escape");</script>"# }>'
impl Escape<{ r#"<script>alert("Escape");</script>"# }> {
    pub fn f() {}
}
