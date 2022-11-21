pub struct FileCount {
    pub path: String,
    pub dir: bool,
    pub lines: Option<usize>,
    pub words: Option<usize>,
    pub bytes: Option<usize>,
}

impl FileCount {
    pub fn new(path: String) -> Self {
        Self {
            path,
            dir: false,
            lines: None,
            words: None,
            bytes: None,
        }
    }
}

pub fn get_bytes(s: &str) -> usize {
    s.len()
}

pub fn get_words(s: &str) -> usize {
    s.split_whitespace().collect::<Vec<_>>().len()
}

pub fn get_lines(s: &str) -> usize {
    s.split('\n').collect::<Vec<_>>().len() - 1
}

#[allow(dead_code)]
pub fn get_chars(s: &str) -> usize {
    s.chars().collect::<Vec<_>>().len()
}
