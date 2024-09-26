use std::{io::Read, sync::LazyLock};

static DEFAULT_TEXTS: &[&str] = &[
    "fn main() {}",
    "println!(\"Hello World\");",
    "std::thread::spawn(|| {});",
    "#[derive(Default, Debug)]",
    "self.counter += 1;",
    "let mut game = Game::new(\"test\");",
];

static TEXTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let default = DEFAULT_TEXTS
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let Ok(mut f) = std::fs::File::open("lines") else {
        return default;
    };
    let mut s = String::new();
    let Ok(size) = f.read_to_string(&mut s) else {
        return default;
    };
    if size == 0 {
        return default;
    }
    s.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
});

#[derive(Default, Debug)]
pub struct Loader {
    counter: usize,
}

impl Loader {
    pub fn load_new_text(&mut self) -> String {
        let result = &TEXTS[self.counter];
        if TEXTS.len() > 1 {
            self.counter += 1;
            self.counter %= TEXTS.len();
        }
        result.clone()
    }
}
