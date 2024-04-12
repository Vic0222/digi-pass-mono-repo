use tracing_subscriber::fmt;

fn main() {

    let format = fmt::format().without_time().with_target(false).compact();

    tracing_subscriber::fmt().event_format(format).init();
    tracing::info!("Starting Inventory Keeper!");

    
}
