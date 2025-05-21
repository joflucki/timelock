use std::error::Error;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    *,
};

pub struct App {
    terminal: DefaultTerminal,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            terminal: init(),
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self) -> Result<(), ()> {
        while !self.exit {
            let app_ref = self;
            self.terminal.draw(|frame| draw_app(app_ref, frame));
            self.handle_events()?;
        }
        Ok(())
    }
}

// Free function to avoid borrowing self in closure
fn draw_app(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

fn handle_events(&mut self) -> Result<(), ()> {
    Ok(())
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), "Yo".yellow()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
