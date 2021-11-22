use structopt::StructOpt;
use tracing::Level;
use ws_tool::ConnBuilder;

/// websocket client connect to binance futures websocket
#[derive(StructOpt)]
struct Args {
    /// channel name, such as btcusdt@depth20
    channels: Vec<String>,

    /// proxy setting
    #[structopt(long)]
    proxy: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .finish();
    let args = Args::from_args();
    let channels = args.channels.join("/");
    let mut builder = ConnBuilder::new(&format!(
        "wss://fstream.binance.com/stream?streams={}",
        channels
    ));
    if let Some(proxy) = args.proxy {
        builder = builder.proxy(&proxy)
    }
    let mut client = builder.build().await.unwrap();
    client.handshake().await.unwrap();

    while let Some(Ok(resp)) = client.read().await {
        let msg = String::from_utf8(resp.payload_data_unmask().to_vec()).unwrap();
        println!("{}", msg.trim());
    }
    Ok(())
}
