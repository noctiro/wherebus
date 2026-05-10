fn main() {
    #[cfg(not(target_os = "android"))]
    tracing_subscriber::fmt()
        .with_env_filter("wherebus=debug")
        .init();
}
