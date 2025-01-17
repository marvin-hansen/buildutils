use warp::Filter;

mod handlers;
mod types;

const PORT: u16 = 4242;

#[tokio::main]
async fn main() {
    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handlers::health_handler);

    let greet = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(handlers::hello_handler);

    let routes = health_check.or(greet);

    print_start_header_simple("Sample Service", "0.0.0.0:4242/");
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
}

fn print_start_header_simple(service_name: &str, service_addr: &str) {
    println!();
    println!("||  {}  ||", service_name);
    println!("==========================================");
    println!("Service on endpoint: {}", service_addr);
    println!("==========================================");
    println!();
}
