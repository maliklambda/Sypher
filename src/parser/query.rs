use crate::constants::special_chars::{QUERY_SEPARATOR, SPACE};

#[derive(Debug, Clone)]
pub struct Query<'a> {
    pub original: &'a str,
    pub current: &'a str,
    pub offset: usize,
    pub generated_names: Vec<String>,
}

impl Query<'_> {
    pub fn from_str(s: &str) -> Query {
        Query {
            original: s,
            current: s,
            offset: 0,
            generated_names: vec![],
        }
    }

    pub fn prepare(&mut self) {
        self.trim_left();
        self.current = self
            .current
            .strip_suffix(QUERY_SEPARATOR)
            .unwrap_or(self.current);
    }

    pub fn trim_left(&mut self) {
        let whitespace_len = self
            .current
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum();
        self.current = &self.current[whitespace_len..];
        self.offset += whitespace_len;
    }

    pub fn trim_left_str<'a>(&mut self, remove: &'a str) -> Option<&'a str> {
        self.trim_left();
        if self.current.starts_with(remove) {
            self.current = &self.current[remove.len()..];
            self.offset += remove.len();
            Some(remove)
        } else {
            None
        }
    }

    pub fn trim_left_char(&mut self, remove: char) -> Option<()> {
        self.trim_left();
        if self.current.starts_with(remove) {
            self.current = &self.current[1..];
            self.offset += 1;
            Some(())
        } else {
            None
        }
    }

    pub fn to_next_space(&mut self) -> Option<&str> {
        self.to_next_char(SPACE)
    }

    pub fn to_next_char(&mut self, c: char) -> Option<&str> {
        let (content, query_rest) = self.current.split_once(c)?;
        self.current = query_rest;
        // self.current = &self.current[content.len()..];
        self.offset += content.len() + 1; // +1 for character length
        Some(content)
    }

    pub fn to_next_str(&mut self, s: &str) -> Option<&str> {
        let (content, query_rest) = self.current.split_once(s)?;
        self.current = query_rest;
        // self.current = &self.current[content.len()..];
        self.offset += content.len() + 1; // +1 for character length
        Some(content)
    }

    pub fn to_end(&mut self) -> &str {
        self.offset += self.current.len();
        let s = self.current;
        self.current = "";
        s
    }

    pub fn generate_uuid(&mut self) -> &str {
        let new_uuid = uuid::Uuid::new_v4();
        self.generated_names.push(new_uuid.to_string());
        self.generated_names.last().unwrap()
    }
}

impl std::fmt::Display for Query<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QUERY: \"{}\" ({} from start)",
            self.current, self.offset
        )
    }
}
