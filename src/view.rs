use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::constants::Screen;
use crate::state::State;

#[derive(Debug, Default)]
pub struct View {}

impl View {
    pub fn draw<B: Backend>(
        terminal: &mut Terminal<B>,
        state: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| {
            let area = frame.size();

            match state.screen {
                Screen::Main => View::draw_main_scene(frame, area, state),
                Screen::Add => View::draw_add_task_scene(frame, area, state),
            }
        })?;
        Ok(())
    }

    fn draw_main_scene(frame: &mut Frame, area: Rect, state: &State) {
        let selected_line = state.line;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        let items: Vec<Span> = state
            .task_list
            .iter()
            .enumerate()
            .map(|e| {
                let checkbox = if e.1.status == "completed" {
                    '✓'
                } else {
                    ' '
                };
                let content = format!(" [{}] {} :: {}", checkbox, e.1.title, e.1.status);
                if e.0 as i32 == selected_line {
                    Span::styled(content, Style::default().bg(Color::LightYellow))
                } else {
                    Span::raw(content)
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Task List"))
            .style(Style::default().fg(Color::White));
        frame.render_widget(list, chunks[0]);

        let bottom_text =
            Paragraph::new("esc: Exit, a: Add, x: Remove, enter: Check/Uncheck, ↑: Up, ↓: Down ")
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::NONE));

        frame.render_widget(bottom_text, chunks[1]);
    }

    fn draw_add_task_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        let content = &state.input;

        let input_field = Paragraph::new(String::from(content))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Write the task, max 80 characters"),
            );

        let bottom_text = Paragraph::new("esc: Cancel, enter: Save")
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(input_field, chunks[0]);
        frame.render_widget(bottom_text, chunks[1]);
    }
}
