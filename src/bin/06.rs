use std::{io, time::Duration};

use advent_of_code::util::fast_parse;
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, canvas::Canvas},
};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let operators_start = input
        .iter()
        .position(|&c| (c == b'+') | (c == b'*'))
        .unwrap();
    let operators: Vec<_> = input[operators_start..]
        .iter()
        .filter_map(|&c| match c {
            b'+' => Some(true),
            b'*' => Some(false),
            _ => None,
        })
        .collect();
    let mut partials = Vec::with_capacity(operators.len());
    let mut numbers_slice = &input[..operators_start];
    // first line
    while numbers_slice[0] != b'\n' {
        let (num, rem) = fast_parse::<u64>(numbers_slice);
        partials.push(num);
        numbers_slice = &rem[rem.iter().position(|&c| c != b' ').unwrap()..];
    }
    numbers_slice = &numbers_slice[1 + numbers_slice[1..]
        .iter()
        .position(|&c| c != b' ')
        .unwrap_or(0)..];
    while !numbers_slice.is_empty() {
        for (partial, &op) in partials.iter_mut().zip(operators.iter()) {
            let (num, rem) = fast_parse::<u64>(numbers_slice);
            *partial = if op { *partial + num } else { *partial * num };
            numbers_slice = &rem[rem.iter().position(|&c| c != b' ').unwrap()..];
        }
        numbers_slice = &numbers_slice[1 + numbers_slice[1..]
            .iter()
            .position(|&c| c != b' ')
            .unwrap_or(0)..];
    }
    Some(partials.iter().sum())
}

fn rotate_input(input: &[u8]) -> Vec<u8> {
    let mut rotated = Vec::with_capacity(input.len());
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    for x in 0..width - 1 {
        rotated.push(input[width * (height - 1) + x]);
        for y in 0..height - 1 {
            rotated.push(input[y * width + x]);
        }
        rotated.push(b'\n');
    }
    rotated
}

fn skip_spaces(input: &[u8]) -> &[u8] {
    &input[input.iter().position(|&c| c != b' ').unwrap_or(0)..]
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotated_input = rotate_input(input.as_bytes());
    let mut remainder = rotated_input.as_slice();
    let mut sum = 0u64;
    while !remainder.is_empty() {
        let operator = remainder[0] == b'+';
        // get to the first number and initialize the partial with it
        remainder = skip_spaces(&remainder[1..]);
        let (mut partial, rem) = fast_parse::<u64>(remainder);
        // end of number to start of next line
        remainder = &skip_spaces(rem)[1..];
        // parse the rest of the numbers in the column
        loop {
            // start of line to start of number
            remainder = skip_spaces(remainder);
            if remainder.is_empty() || remainder[0] == b'\n' {
                if !remainder.is_empty() {
                    // skip new line
                    remainder = &remainder[1..];
                }
                // seperating empty line spotted
                sum += partial;
                break;
            }
            let (num, rem) = fast_parse::<u64>(remainder);
            remainder = &skip_spaces(rem)[1..];
            partial = if operator {
                partial + num
            } else {
                partial * num
            };
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
