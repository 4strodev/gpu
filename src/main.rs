fn main() {
    match gpu::run() {
        Ok(it) => it,
        Err(err) => panic!("{}", err),
    };
}
