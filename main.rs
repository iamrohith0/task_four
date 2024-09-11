use chrono::{Local, TimeZone};  
use chrono_tz::Asia::Kolkata;   

fn main() {
    let name = "Rohith R D";  // Replace with your actual name 
    let now = Kolkata.from_utc_datetime(&Local::now().naive_utc());  
    println!("Hello {}, right now the time is {}", name, now.format("%Y-%m-%d %H:%M:%S"));
}


