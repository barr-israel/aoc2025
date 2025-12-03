use std::{io, time::Duration};

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, canvas::Canvas},
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut sum = 0u64;
    let mut lines = 0u64;
    while !input.is_empty() {
        lines += 1;
        let mut tens = input[0];
        let mut ones = b'0';
        input = &input[1..];
        while input[0] != b'\n' {
            let digit = input[0];
            if digit > tens && input[1] != b'\n' {
                tens = digit;
                // will be corrected next iteration
                ones = b'0';
            } else if digit > ones {
                ones = digit;
            }
            input = &input[1..];
        }
        // skip \n
        input = &input[1..];
        sum += tens as u64 * 10 + ones as u64;
    }
    sum -= b'0' as u64 * 11 * lines;
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    const DIGITS: usize = 12;
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|mut line| {
            if line.is_empty() {
                return 0;
            }
            let mut sum = 0u64;
            for digit in (0..DIGITS).rev() {
                let mut max = line[0];
                let mut max_index = 0;
                // we need to reserve `digit` digits at the end of the line for the rest of the process
                for (index, &digit) in line[0..line.len() - digit].iter().enumerate() {
                    if digit > max {
                        max = digit;
                        max_index = index;
                    }
                    // if max == b'9' {
                    //     break;
                    // }
                }
                sum = sum * 10 + (max - b'0') as u64;
                line = &line[max_index + 1..];
            }
            sum
        })
        .sum::<u64>()
        .into()
}

#[derive(Debug)]
struct Part1App<'a> {
    input: &'a [u8],
}

impl<'a> Part1App<'a> {
    fn reset(&mut self) {
        *self = Self::new(self.input);
    }

    fn new(input: &'a [u8]) -> Self {
        Self { input }
    }
}

fn part_one_tick(_state: &mut Part1App<'_>) {}

impl Widget for &Part1App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Part One".bold());
        let status = Text::from(format!("Length: {}", self.input.len()));
        let block = Block::bordered().title(title.centered());
        let [status_area, dial_area] =
            Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(block.inner(area));
        block.render(area, buf);
        Canvas::default()
            .x_bounds([-200f64, 200f64])
            .y_bounds([-100f64, 100f64])
            .block(Block::bordered().border_type(BorderType::Double))
            .paint(|_ctx| {})
            .render(dial_area, buf);
        Paragraph::new(status).centered().render(status_area, buf);
    }
}

#[derive(Debug)]
struct Part2App<'a> {
    input: &'a [u8],
}

impl<'a> Part2App<'a> {
    fn reset(&mut self) {
        *self = Self::new(self.input);
    }

    fn new(input: &'a [u8]) -> Self {
        Self { input }
    }
}

fn part_two_tick(_state: &mut Part2App<'_>) {}

impl Widget for &Part2App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Part Two".bold());
        let status = Text::from(format!("Length: {}", self.input.len()));
        let block = Block::bordered().title(title.centered());
        let [status_area, dial_area] =
            Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(block.inner(area));
        block.render(area, buf);
        Canvas::default()
            .x_bounds([-200f64, 200f64])
            .y_bounds([-100f64, 100f64])
            .block(Block::bordered().border_type(BorderType::Double))
            .paint(|_ctx| {})
            .render(dial_area, buf);
        Paragraph::new(status).centered().render(status_area, buf);
    }
}

pub fn part_one_tui(input: &str) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = Part1App::new(input.as_bytes());
    let mut paused = false;
    loop {
        terminal.clear()?;
        part_one_tick(&mut state);
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(&state, frame.area());
        })?;
        if !event::poll(Duration::from_millis(8))? && !paused {
            continue;
        }
        if let Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') => {
                    ratatui::restore();
                    return Ok(());
                }
                KeyCode::Char('r') => state.reset(),
                KeyCode::Char('p') => paused = !paused,
                _ => {}
            }
        }
    }
}

pub fn part_two_tui(input: &str) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = Part2App::new(input.as_bytes());
    let mut paused = false;
    loop {
        terminal.clear()?;
        part_two_tick(&mut state);
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(&state, frame.area());
        })?;
        if !event::poll(Duration::from_millis(8))? && !paused {
            continue;
        }
        if let Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') => {
                    ratatui::restore();
                    return Ok(());
                }
                KeyCode::Char('r') => state.reset(),
                KeyCode::Char('p') => paused = !paused,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
