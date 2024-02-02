use eframe::egui::widgets::text_edit::TextBuffer;

pub struct MyTextBuffer<'a> {
    string: &'a mut String,
}
impl <'a>MyTextBuffer<'a> {
    pub fn new(string: &'a mut String) -> Self {
        return MyTextBuffer { string: string };
    }
}

impl<'a> TextBuffer for MyTextBuffer<'a> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.string.as_ref()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let byte_idx = self.byte_index_from_char_index(char_index);

        // Then insert the string
        self.string.insert_str(byte_idx, text);

        text.chars().count()
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        assert!(char_range.start <= char_range.end);

        // Get both byte indices
        let byte_start = self.byte_index_from_char_index(char_range.start);
        let byte_end = self.byte_index_from_char_index(char_range.end);

        // Then drain all characters within this range
        self.string.drain(byte_start..byte_end);
    }

    fn clear(&mut self) {
        self.string.clear();
    }

    fn replace(&mut self, text: &str) {
        *self.string = text.to_owned();
    }

    fn take(&mut self) -> String {
        std::mem::take(self.string)
    }
}
