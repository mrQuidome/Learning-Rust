// the integer is put on thge stack, it is fixed size
// the string is put on the heap, it has a variable size
fn get_parameters(_n: i32, _s: String) {

}

fn main() {
    get_parameters(12, String::from("test"));
}