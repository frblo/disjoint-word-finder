use word_handler::signature;

mod word_handler;

fn main() {
    let str = "äppleé".to_string();
    print!("{:?}", signature(str));
}
