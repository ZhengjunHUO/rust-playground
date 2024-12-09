trait Foo {
    fn do_foo(&self);
}

trait Bar {
    fn do_bar(&self);
}

struct SomeFoo {}
struct SomeBar {}
struct AnotherFoo {}

impl Foo for SomeFoo {
    fn do_foo(&self) {
        println!("SomeFoo is speaking");
    }
}

impl Foo for AnotherFoo {
    fn do_foo(&self) {
        println!("AnotherFoo is speaking");
    }
}

impl Bar for SomeBar {
    fn do_bar(&self) {
        println!("SomeBar is speaking");
    }
}

trait AggrTrait {
    fn do_something(&self);
}

struct Aggr<T1, T2> {
    mem1: T1,
    mem2: T2,
}

impl<T1: Foo, T2: Bar> AggrTrait for Aggr<T1, T2> {
    fn do_something(&self) {
        self.mem1.do_foo();
        self.mem2.do_bar();
    }
}

fn handle(instance: &dyn AggrTrait) {
    instance.do_something();
}

fn main() {
    let instance: Box<dyn AggrTrait> = match std::env::var("USE_ANOTHER_FOO") {
        Ok(_) => Box::new(Aggr {
            mem1: AnotherFoo {},
            mem2: SomeBar {},
        }),
        Err(_) => Box::new(Aggr {
            mem1: SomeFoo {},
            mem2: SomeBar {},
        }),
    };

    handle(&*instance);
}
