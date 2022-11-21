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

pub enum EnumCount {
    Lines,
    Words,
    Bytes,
}

pub fn set_count(file_count: &mut FileCount, s: &String, count: EnumCount) {
    match count {
        EnumCount::Lines => match file_count.dir {
            true => {
                let file_lines = get_lines(s);
                file_count.lines = Some(file_lines);
            }
            false => {
                file_count.lines = Some(0);
            }
        },
        EnumCount::Words => match file_count.dir {
            true => {
                let file_words = get_words(&s);
                file_count.words = Some(file_words);
            }
            false => {
                file_count.words = Some(0);
            }
        },
        EnumCount::Bytes => match file_count.dir {
            true => {
                let file_bytes = get_bytes(&s);
                file_count.bytes = Some(file_bytes);
            }
            false => {
                file_count.bytes = Some(0);
            }
        },
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
