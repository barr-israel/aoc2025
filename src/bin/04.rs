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

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let mut count = (input[0] == b'@') as u64 + (input[input.len() - 1] == b'@') as u64;
    count += (1..width - 1)
        .filter(|&pos| {
            if input[pos] != b'@' {
                return false;
            }
            let mut count = 0u8;
            count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
            count += (input[pos + width - 1] == b'@') as u8
                + (input[pos + width] == b'@') as u8
                + (input[pos + width + 1] == b'@') as u8;
            count < 4
        })
        .count() as u64;
    if input[width] == b'@'
        && ((input[0] == b'@') as u8
            + (input[1] == b'@') as u8
            + (input[width + 1] == b'@') as u8
            + (input[2 * width] == b'@') as u8
            + (input[2 * width + 1] == b'@') as u8)
            < 4
    {
        count += 1;
    }
    count += (width + 1..input.len() - width - 1)
        .filter(|&pos| {
            if input[pos] != b'@' {
                return false;
            }
            let mut count = 0u8;
            count += (input[pos - width - 1] == b'@') as u8
                + (input[pos - width] == b'@') as u8
                + (input[pos - width + 1] == b'@') as u8;
            count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
            count += (input[pos + width - 1] == b'@') as u8
                + (input[pos + width] == b'@') as u8
                + (input[pos + width + 1] == b'@') as u8;
            count < 4
        })
        .count() as u64;
    count += (input.len() - width..input.len() - 1)
        .filter(|&pos| {
            if input[pos] != b'@' {
                return false;
            }
            let mut count = 0u8;
            count += (input[pos - width - 1] == b'@') as u8
                + (input[pos - width] == b'@') as u8
                + (input[pos - width + 1] == b'@') as u8;
            count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
            count < 4
        })
        .count() as u64;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.to_owned().into_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let mut total_count = 0u64;
    loop {
        let mut count = 0u64;
        if input[0] == b'@' {
            input[0] = b'x';
            count += 1;
        }
        if input[1] == b'@' {
            input[1] = b'x';
            count += 1;
        }
        count += (1..width - 1)
            .filter(|&pos| {
                if input[pos] != b'@' {
                    return false;
                }
                let mut count = 0u8;
                count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
                count += (input[pos + width - 1] == b'@') as u8
                    + (input[pos + width] == b'@') as u8
                    + (input[pos + width + 1] == b'@') as u8;
                if count < 4 {
                    input[pos] = b'x';
                    return true;
                }
                false
            })
            .count() as u64;
        if input[width] == b'@'
            && ((input[0] == b'@') as u8
                + (input[1] == b'@') as u8
                + (input[width + 1] == b'@') as u8
                + (input[2 * width] == b'@') as u8
                + (input[2 * width + 1] == b'@') as u8)
                < 4
        {
            input[width] = b'x';
            count += 1;
        }
        count += (width + 1..input.len() - width - 1)
            .filter(|&pos| {
                if input[pos] != b'@' {
                    return false;
                }
                let mut count = 0u8;
                count += (input[pos - width - 1] == b'@') as u8
                    + (input[pos - width] == b'@') as u8
                    + (input[pos - width + 1] == b'@') as u8;
                count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
                count += (input[pos + width - 1] == b'@') as u8
                    + (input[pos + width] == b'@') as u8
                    + (input[pos + width + 1] == b'@') as u8;
                if count < 4 {
                    input[pos] = b'x';
                    return true;
                }
                false
            })
            .count() as u64;
        count += (input.len() - width..input.len() - 1)
            .filter(|&pos| {
                if input[pos] != b'@' {
                    return false;
                }
                let mut count = 0u8;
                count += (input[pos - width - 1] == b'@') as u8
                    + (input[pos - width] == b'@') as u8
                    + (input[pos - width + 1] == b'@') as u8;
                count += (input[pos - 1] == b'@') as u8 + (input[pos + 1] == b'@') as u8;
                if count < 4 {
                    input[pos] = b'x';
                    return true;
                }
                false
            })
            .count() as u64;
        total_count += count;
        if count == 0 {
            break;
        }
    }
    Some(total_count)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
