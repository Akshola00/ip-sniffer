mod concurrency;
mod ip_sniffer;
fn main() {
    // ip_sniffer::ip_sniffer();
    // concurrency::channel_impl();
    for i in 0..4 {
        println!("{i}");
    }
}
