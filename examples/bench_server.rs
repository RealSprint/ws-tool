use clap::Parser;
use tracing_subscriber::util::SubscriberInitExt;
use ws_tool::{
    codec::{default_handshake_handler, WsBytesCodec},
    frame::OpCode,
    stream::BufStream,
    ServerBuilder,
};

/// websocket client connect to binance futures websocket
#[derive(Parser)]
struct Args {
    /// server host
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    /// server port
    #[arg(short, long, default_value = "9000")]
    port: u16,

    /// level
    #[arg(short, long, default_value = "info")]
    level: tracing::Level,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    tracing_subscriber::fmt::fmt()
        .with_max_level(args.level)
        .finish()
        .try_init()
        .expect("failed to init log");
    tracing::info!("binding on {}:{}", args.host, args.port);
    let listener = std::net::TcpListener::bind(format!("{}:{}", args.host, args.port)).unwrap();
    loop {
        let (stream, addr) = listener.accept().unwrap();
        stream.set_nodelay(true).unwrap();
        std::thread::spawn(move || {
            tracing::info!("got connect from {:?}", addr);
            let mut server =
                ServerBuilder::accept(stream, default_handshake_handler, |req, stream| {
                    WsBytesCodec::factory(req, BufStream::new(stream))
                })
                .unwrap();

            loop {
                let mut msg = server.receive().unwrap();
                if msg.code == OpCode::Close {
                    break;
                }
                server.send(&mut msg.data[..]).unwrap();
            }
            tracing::info!("one conn down");
        });
    }
}
