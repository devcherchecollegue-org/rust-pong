pub fn init() {
    dotenv::dotenv().ok();
    std::env::var("RUST_LOG").expect("error,warn,info,trace");
    std::env::var("RUST_BACKTRACE").expect("1 or 0");
    env_logger::init();
    log::info!("Logger is up");
}
