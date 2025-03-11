pub trait Handler {
    fn run(&self);
}

pub fn run<T: Handler + std::str::FromStr>() {
    let value = std::env::var("_HANDLER").expect("_HANDLER environment variable");
    T::from_str(&value)
        .unwrap_or_else(|_| panic!("_HANDLER value error: {} not registered", &value))
        .run()
}
