mod utils;

use tracing::{span, Level,info};
use std::{io::stdout, net::SocketAddr};

use warp::Filter;


#[tokio::main]
async fn main() {

    let (core,_guard) = utils::setup_backbone();

    let path = warp::path!( String / "auth")
    .map(|auth| {
        format!("ok/authorized")
    })
    .or(
        // Green - empty/ready to receive/idle
        // url: /<key>/green/<status_flags>/<message>
        // emitting this voids transfers
        warp::path!( String / "green").map(
            |auth| {
                // should return almost all command and control
                // commands in this response
                // also try to bind nodes to jobs at this time

                // apon a job being bound it
                // should put the controller and node into
                // yellow state, instruct it to receve items
                // and craft apon receving all units
                format!("ok/")
            }
        )
    ).or(
        // amber - trying to return to ready/inbetween states, cannot fufill
        // url: /<key>/amber/<status_flags>/<message>
        // emitting this fufills transfers
        warp::path!( String / "amber" ).map(|auth| {
                format!("ok/")
            })
    ).or(
        // Red - fault of any kind
        // url: /<key>/red/<status_flags>/<message>
        warp::path!( String / "red" ).map(|auth| {
            format!("ok/")
        })
    ).or(
        // Yellow - disabled/hibernate
        // url: /<key>/yellow/<status_flags>
        // should respond with ok or no auth
        // emitting this voids transfers
        warp::path!( String / "yellow" / String ).map(|auth,status| {
            format!("ok/")
        })
    ).or(
        // blue - waiting to send order
        // url: /<key>/blue/<status_flags>/
        // should respond with a tp tube pipe and an order ID
        // emitting this continues transfers
        warp::path!( String / "blue" / String ).map(|auth,status| {
            format!("ok/")
        })
    ).or(
        // blue - waiting to receve order
        // url: /<key>/purple/<status_flags>/
        // waiting to receve order from 1 or more blue nodes
        warp::path!( String / "purple" / String ).map(|auth,status| {
            format!("ok/")
        })
    ).or(
        // a silly little API path to "wave hi"
        // url: /<key>/purple/<status_flags>/
        warp::path!( String / "wavehi" / String ).map(|auth,message| {
            println!("wavehi: {message}");
            format!("ok/")
        })
    );

    let bind_address: SocketAddr = core.config.address.parse().unwrap();

    let logspan = span!(Level::INFO,"HTTP Serve");
    let _logspan = logspan.enter();

    info!("Serving website");

    println!("serving at: {}",core.config.address);

    warp::serve(path)
    .run(bind_address).await;
}