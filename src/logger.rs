pub fn init() {
    dotenv::dotenv().ok();
    std::env::var("RUST_LOG")
        .expect("RUST_LOG environment variable must contain at least one of error,warn,info,trace");
    std::env::var("RUST_BACKTRACE").expect("RUST_BACKTRACE environment variable must be 1 or 0");
    env_logger::init();
    log::info!("Logger is up");
}
