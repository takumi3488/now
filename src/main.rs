use chrono::Local;

fn main() {
    let text = Local::now().format("%Y/%m/%d %H:%M:%S");
    println!("{}", text);
}
