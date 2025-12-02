use std::{
    io,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::Duration,
};

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

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let invalid = AtomicU64::new(0u64);
    rayon::scope(|s| {
        while !input.is_empty() {
            let (start, rem) = fast_parse::<u64>(input);
            let (end, rem) = fast_parse::<u64>(&rem[1..]);
            input = &rem[1..];
            let invalid_ref = &invalid;
            s.spawn(move |_| count_invalid(start, end, invalid_ref));
        }
    });
    Some(invalid.load(Relaxed))
}

fn count_invalid(start: u64, end: u64, invalid_counter: &AtomicU64) {
    invalid_counter.fetch_add(
        (start..end + 1)
            .filter(|&num| {
                let half_divisor = 10u64.pow(num.ilog10().div_ceil(2));
                num / half_divisor == num % half_divisor
            })
            .sum(),
        Relaxed,
    );
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let invalid = AtomicU64::new(0u64);
    rayon::scope(|s| {
        while !input.is_empty() {
            let (start, rem) = fast_parse::<u64>(input);
            let (end, rem) = fast_parse::<u64>(&rem[1..]);
            input = &rem[1..];
            let invalid_ref = &invalid;
            s.spawn(move |_| count_invalid_part_two(start, end, invalid_ref));
        }
    });
    Some(invalid.load(Relaxed))
}

fn count_invalid_part_two(start: u64, end: u64, invalid_counter: &AtomicU64) {
    invalid_counter.fetch_add(
        (start..end + 1)
            .filter(|&num| {
                if num < 10 {
                    return false;
                }
                let num_string = num.to_string().into_bytes();
                for chunk_size in (1..num_string.len().div_ceil(2) + 1).rev() {
                    let mut iter = num_string.chunks(chunk_size);
                    let first = iter.next().unwrap();
                    if iter.all(|c| c == first) {
                        return true;
                    }
                }
                false
            })
            .sum(),
        Relaxed,
    );
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

#[allow(unused_variables)]
fn part_one_tick(state: &mut Part1App<'_>) {}

#[allow(unused_variables)]
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
            .paint(|ctx| {})
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

#[allow(unused_variables)]
fn part_two_tick(state: &mut Part2App<'_>) {}

#[allow(unused_variables)]
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
            .paint(|ctx| {})
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
