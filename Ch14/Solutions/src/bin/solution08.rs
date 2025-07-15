struct StaticHolder {
    data: &'static str,
}

fn main() {
    let holder = StaticHolder {
        data: "static text"
    };
    println!("{}", holder.data);

    // This will not compile
    // let heap_data = "heap text".to_string();
    // let holder = Holder {
    //     data: &heap_data
    // };
    // println!("{}", holder.data);
}
