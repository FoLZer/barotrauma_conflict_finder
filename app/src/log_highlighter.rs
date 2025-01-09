use std::ops::Range;

use iced::Color;

#[derive(Debug)]
pub struct Highlighter {
    current_line: usize,
}

impl iced::widget::text::Highlighter for Highlighter {
    type Settings = ();
    type Highlight = Option<Color>;

    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(_settings: &Self::Settings) -> Self {
        Highlighter { current_line: 0 }
    }

    fn update(&mut self, _new_settings: &Self::Settings) {}

    fn change_line(&mut self, _line: usize) {}

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        self.current_line += 1;

        //This is fine for current log format
        let Some(spl) = line.split_once(' ') else {
            return Box::new(std::iter::empty());
        };
        let c = match spl.0 {
            "ERROR" => Some(Color::from_rgb8(205, 0, 0)),
            "WARN" => Some(Color::from_rgb8(205, 205, 0)),
            "INFO" => Some(Color::from_rgb8(0, 205, 205)),
            "DEBUG" => Some(Color::from_rgb8(205, 0, 205)),
            "TRACE" => None,
            _ => None,
        };

        Box::new(std::iter::once((0..spl.0.len(), c)))
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}
