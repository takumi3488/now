use chrono::Local;

fn main() {
    let text = Local::now().format("%Y/%m/%d %H:%M:%S (日本時間)");
    println!("{}", text);
}
