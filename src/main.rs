extern "C" {
    fn throws();
    fn aborts();
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 || (args[1] != "abort" && args[1] != "throw") {
        println!("usage:\n\t{} abort\n\t{} throw", args[0], args[0]);
        return;
    }

    if args[1] == "abort" {
        unsafe { aborts() };
    } else {
        unsafe { throws() };
    }
}
