pub(crate) fn print_start_header_simple(service_name: &str, service_addr: &str) {
    println!();
    println!("||  {}  ||", service_name);
    println!("==========================================");
    println!("Service on endpoint: {}", service_addr);
    println!("==========================================");
    println!();
}
