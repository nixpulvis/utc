fn main() {
    let now = time::now_utc();

    if let Ok(fmt) = now.strftime("%F %H:%M:%S") {
        println!("{}", fmt);
    } else {
        println!("Error formatting time: {:?}", now);
    }
}
