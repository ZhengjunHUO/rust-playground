use macro_test::MyMacro;
use macro_test_derive::MyMacro;

#[derive(MyMacro)]
struct Huo;

fn main() {
    Huo::my_macro();
}
