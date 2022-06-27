extern crate warp;
extern crate tokio;
extern crate fern;

use warp::Filter;

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
    .format(|out, message, record| {
	out.finish(format_args!(
	    "{}[{}][{}] {}",
	    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
	    record.target(),
	    record.level(),
	    message
	))
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    .apply();

    let hlog = warp::log("hlog");
    let glog = warp::log("glog");
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name))
        .with(hlog);
        
    let goodbye = warp::path!("goodbye")
        .map(|| format!("goodbye"))
        .with(glog);

    let routes = hello.or(goodbye);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
