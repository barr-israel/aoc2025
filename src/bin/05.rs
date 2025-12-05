use std::{io, time::Duration, u64};

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

advent_of_code::solution!(5);

#[derive(Debug)]
struct MyRange {
    start: u64,
    end: u64,
}

impl MyRange {
    fn len(&self) -> u64 {
        1 + self.end - self.start
    }
    fn contains(&self, other: &u64) -> bool {
        self.start <= *other && self.end >= *other
    }
}

fn parse_fresh(mut input: &[u8]) -> (Vec<MyRange>, &[u8]) {
    let mut fresh = vec![];
    while input[0] != b'\n' {
        let (start, rem) = fast_parse(input);
        let (end, rem) = fast_parse(&rem[1..]);
        fresh.push(MyRange { start, end });
        input = &rem[1..];
    }
    (fresh, input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh, rem) = parse_fresh(input.as_bytes());
    let mut input = rem;
    let mut fresh_count = 0u64;
    while !input.is_empty() {
        let (ingridient, rem) = fast_parse::<u64>(input);
        if fresh.iter().any(|r| r.contains(&ingridient)) {
            fresh_count += 1;
        }
        input = &rem[1..];
    }
    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut fresh, _) = parse_fresh(input.as_bytes());
    fresh.sort_unstable_by_key(|r| r.start);
    // deduplicate ranges
    let mut fresh_count = 0u64;
    'outer: for i in 0..fresh.len() {
        let (a, b) = fresh.split_at_mut(i + 1);
        let this_range = &mut a[i];
        if this_range.len() == 0 {
            continue;
        }
        for other_range in b {
            if this_range.end < other_range.start {
                fresh_count += this_range.len();
                continue 'outer;
            }
            dedup(this_range, other_range);
        }
        fresh_count += this_range.len();
    }
    Some(fresh_count)
}

fn dedup(r1: &mut MyRange, r2: &mut MyRange) {
    // r2 cant end before r1 starts because the ranges are sorted
    if r1.end < r2.start {
        return;
    }
    r1.start = (r1.start).min(r2.start);
    r1.end = (r1.end).max(r2.end);
    *r2 = MyRange {
        start: u64::MAX,
        end: u64::MAX - 1,
    };
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
