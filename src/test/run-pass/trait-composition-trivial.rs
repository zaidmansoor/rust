// xfail-fast - ICE
trait Foo {
    fn foo();
}

trait Bar : Foo {
    fn bar();
}

fn main() {}


