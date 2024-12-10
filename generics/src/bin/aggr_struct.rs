trait Foo {
    fn do_foo(&self);
}

trait Bar {
    fn do_bar(&self);
}

struct SomeFoo {
    num: u8,
}
struct SomeBar {}
struct AnotherFoo {
    num: u8,
}

impl Foo for SomeFoo {
    fn do_foo(&self) {
        println!("SomeFoo {} is speaking", self.num);
    }
}

impl Foo for AnotherFoo {
    fn do_foo(&self) {
        println!("AnotherFoo {} is speaking", self.num);
    }
}

impl Bar for SomeBar {
    fn do_bar(&self) {
        println!("SomeBar is speaking");
    }
}

trait AggrTrait {
    fn do_something(&self);
    fn get_bar_ref(&self) -> &dyn Bar;
}

struct Aggr<T1, T2> {
    mem1: Vec<T1>,
    mem2: Option<T2>,
}

impl<T1: Foo, T2: Bar> AggrTrait for Aggr<T1, T2> {
    fn do_something(&self) {
        self.mem1.iter().for_each(|elem| elem.do_foo());

        match &self.mem2 {
            Some(obj) => obj.do_bar(),
            None => (),
        }
    }

    fn get_bar_ref(&self) -> &dyn Bar {
        self.mem2.as_ref().unwrap()
    }
}

fn handle(instance: &dyn AggrTrait) {
    instance.do_something();
}

fn new_foo(num: u8) -> SomeFoo {
    SomeFoo { num }
}

fn new_another(num: u8) -> AnotherFoo {
    AnotherFoo { num }
}

fn new_bar() -> SomeBar {
    SomeBar {}
}

fn main() {
    let instance: Box<dyn AggrTrait> = match std::env::var("USE_ANOTHER_FOO") {
        Ok(_) => Box::new(Aggr {
            mem1: vec![new_another(0), new_another(1), new_another(2)],
            mem2: Some(new_bar()),
        }),
        Err(_) => Box::new(Aggr {
            mem1: vec![new_foo(0), new_foo(1), new_foo(2)],
            mem2: Some(new_bar()),
        }),
    };

    instance.get_bar_ref().do_bar();

    handle(&*instance);
}
