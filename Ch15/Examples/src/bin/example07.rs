struct MyStruct;

unsafe trait DangerousTrait {
    fn dangerous_method(&self);
}

unsafe impl DangerousTrait for MyStruct {
    fn dangerous_method(&self) {
        println!("inside dangerous_method");
    }
}

fn main() {
    let my_struct = MyStruct;
    my_struct.dangerous_method();
}
