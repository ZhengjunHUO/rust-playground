trait MyTrait {
    fn init(self, init_data: i32) -> Self;
    fn peek(&self) -> (i32, i32);
}

struct Concrete1 {
    data: i32,
    triple: i32,
}

impl MyTrait for Concrete1 {
    fn init(mut self, init_data: i32) -> Self {
        self.data = init_data;
        self.triple = init_data * 3;
        self
    }

    fn peek(&self) -> (i32, i32) {
        (self.data, self.triple)
    }
}

struct Concrete2 {
    data: i32,
    double: i32,
}

impl MyTrait for Concrete2 {
    fn init(mut self, init_data: i32) -> Self {
        self.data = init_data;
        self.double = init_data * 2;
        self
    }

    fn peek(&self) -> (i32, i32) {
        (self.data, self.double)
    }
}

struct ClientWrapper<T> {
    client: T,
}

fn init<T: MyTrait>(client_w: ClientWrapper<T>, init_data: i32) -> ClientWrapper<T> {
    ClientWrapper {
        client: client_w.client.init(init_data),
    }
}

fn peek<T: MyTrait>(client_w: &ClientWrapper<T>) {
    println!("Data inside client: {:?}", client_w.client.peek());
}

fn main() {
    let client1 = ClientWrapper {
        client: Concrete1 { data: 0, triple: 0 },
    };
    let client2 = ClientWrapper {
        client: Concrete2 { data: 0, double: 0 },
    };

    let client1_after_init = init(client1, 123);
    let client2_after_init = init(client2, 456);

    peek(&client1_after_init);
    peek(&client2_after_init);
}
