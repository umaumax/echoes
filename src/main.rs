use log::{debug, error, info};
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

#[derive(StructOpt)]
struct Opt {
    #[structopt(default_value("localhost:8080"), help = "bind address")]
    addr: String,

    #[structopt(long = "verbose", help = "verbose flag")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let addr = opt.addr;
    let verbose = opt.verbose;

    let listener = TcpListener::bind(&addr).await?;

    env::set_var("RUST_LOG", "info");
    if verbose {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    info!("Listening on: {}", addr);
    loop {
        let (mut socket, from_addr) = listener.accept().await?;
        debug!("From: {}", from_addr);

        let _ = tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket.read(&mut buf).await.map_err(|e| {
                    error!("{}", e);
                    e
                })?;

                if n == 0 {
                    return Ok::<(), std::io::Error>(());
                }

                socket.write_all(&buf[0..n]).await.map_err(|e| {
                    error!("{}", e);
                    e
                })?;
            }
        });
    }
}
