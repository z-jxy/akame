fn hello(num) {
    return num + 5;
}

fn main() {
    let arg = std::args[1];
    print(arg);
    printd(hello(4));
}
