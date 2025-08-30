mod utils;

use tracing::{span, Level,info};
use std::{io::stdout, net::SocketAddr};

use warp::Filter;


#[tokio::main]
async fn main() {

    let (core,_guard) = utils::setup_backbone();

    let auth = warp::path!( String / "auth" / String / String)
    .map(|_base_path,user,pass| {
        format!("1:authorized")
    });

    let bind_address: SocketAddr = core.config.address.parse().unwrap();

    let logspan = span!(Level::INFO,"HTTP Serve");
    let _logspan = logspan.enter();

    info!("Serving website");

    println!("sering at: {}",core.config.address);

    warp::serve(auth)
    .run(bind_address).await;
}