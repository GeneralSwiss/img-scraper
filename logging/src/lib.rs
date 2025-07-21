use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .init()
}
