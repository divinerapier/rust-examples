trait Foo {}
trait Bar: Sized {}

struct Impl;
impl Foo for Impl {}
impl Bar for Impl {}

fn foo() {
    let x: &dyn Foo = &Impl; // OK
                             // let y: &dyn Bar = &Impl; // error: the trait `Bar` cannot
                             // be made into an object
}
