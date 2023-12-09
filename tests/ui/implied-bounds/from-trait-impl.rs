// check-pass
// known-bug: #109628

trait Trait {
    type Assoc;
}

impl<X: 'static> Trait for (X,) {
    type Assoc = ();
}

struct Foo<T: Trait>(T)
where
    T::Assoc: Clone; // any predicate using `T::Assoc` works here

fn func1(foo: Foo<(&str,)>) {
    let _: &'static str = foo.0.0;
}

trait TestTrait {}

impl<X> TestTrait for [Foo<(X,)>; 1] {}

fn main() {}
