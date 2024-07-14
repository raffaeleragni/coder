static TEXTS: &[&str] = &[
    "println!(\"Hello World\");",
    "fn main() {}",
    "std::thread::spawn(|| {});",
];

#[derive(Default, Debug)]
pub struct Loader {
    counter: usize,
}

impl Loader {
    pub fn load_new_text(&mut self) -> &'static str {
        let result = TEXTS[self.counter];
        self.counter += 1;
        self.counter %= TEXTS.len();
        result
    }
}
