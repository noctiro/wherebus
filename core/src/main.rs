fn main() {
    #[cfg(not(target_os = "android"))]
    {
        use tracing_subscriber::filter;
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;
        tracing_subscriber::registry()
            .with(filter::Targets::new().with_target("wherebus", tracing::Level::DEBUG))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}
