#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MyStruct {
    a: i32,
    b: u32,
    c: *mut MyStruct,
}

impl MyStruct {
    pub fn new(a: i32, b: u32, c: *mut MyStruct) -> MyStruct {
        MyStruct { a, b, c }
    }

    pub fn hello(&self) {
        println!("hello from MyStruct! a: {} b: {}", self.a, self.b);
    }
}

pub fn main() {
    let my_struct = MyStruct::new(1, 2, 0xfffff as *mut MyStruct);
    my_struct.hello();
    println!("{:#?}", my_struct);
}