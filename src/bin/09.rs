use std::{io, time::Duration};

use advent_of_code::util::fast_parse;
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        Block, BorderType, Paragraph, Widget,
        canvas::{self, Canvas, Rectangle},
    },
};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut tiles = vec![];
    while !input.is_empty() {
        let (x, rem) = fast_parse::<u64>(input);
        let (y, rem) = fast_parse::<u64>(&rem[1..]);
        input = &rem[1..];
        tiles.push((x, y));
    }
    tiles
        .iter()
        .enumerate()
        .flat_map(|(idx, a)| {
            tiles[idx + 1..]
                .iter()
                .map(|b| {
                    let (min_x, max_x) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
                    let (min_y, max_y) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
                    (min_x, min_y, max_x + 1, max_y + 1)
                })
                .map(|(x1, y1, x2, y2)| (x2 - x1) * (y2 - y1))
        })
        .max()
}

fn is_clockwise(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> bool {
    (p3.1 - p1.1) * (p2.0 - p1.0) > (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn intersect(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64), p4: (i64, i64)) -> bool {
    (is_clockwise(p1, p3, p4) != is_clockwise(p2, p3, p4))
        && (is_clockwise(p1, p2, p3) != is_clockwise(p1, p2, p4))
}

fn any_lines_crossing(idx1: usize, idx2: usize, tiles: &[(i64, i64)]) -> bool {
    let p1 = tiles[idx1];
    let p2 = tiles[idx2];
    for (idx3, &p3) in tiles.iter().enumerate() {
        if idx3 == idx1 || idx3 == idx2 {
            continue;
        }
        let idx4 = if idx3 == tiles.len() - 1 { 0 } else { idx3 + 1 };
        if idx4 == idx1 || idx4 == idx2 {
            continue;
        }
        let p4 = tiles[idx4];
        if intersect(p1, p2, p3, p4) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut tiles = vec![];
    while !input.is_empty() {
        let (x, rem) = fast_parse::<i64>(input);
        let (y, rem) = fast_parse::<i64>(&rem[1..]);
        input = &rem[1..];
        tiles.push((x, y));
    }
    let mut max_area = 0i64;
    for (idx1, p1) in tiles.iter().enumerate() {
        for (idx2, p2) in tiles[idx1 + 1..].iter().enumerate() {
            let idx2 = idx2 + idx1 + 1;
            let (min_x, max_x) = if p1.0 < p2.0 {
                (p1.0, p2.0)
            } else {
                (p2.0, p1.0)
            };
            let (min_y, max_y) = if p1.1 < p2.1 {
                (p1.1, p2.1)
            } else {
                (p2.1, p1.1)
            };
            if tiles
                .iter()
                .any(|&(x3, y3)| x3 > min_x && x3 < max_x && y3 > min_y && y3 < max_y)
            {
                continue;
            }
            if any_lines_crossing(idx1, idx2, &tiles) {
                continue;
            }
            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Some(max_area as u64)
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
    tiles: Vec<(i64, i64)>,
    idx2: usize,
    idx1: usize,
    valid: bool,
    skip_to_valid: bool,
}

impl<'a> Part2App<'a> {
    fn reset(&mut self) {
        *self = Self::new(self.input);
    }

    fn new(input: &'a [u8]) -> Self {
        let mut cursor = input;
        let mut tiles = vec![];
        while !cursor.is_empty() {
            let (x, rem) = fast_parse::<i64>(cursor);
            let (y, rem) = fast_parse::<i64>(&rem[1..]);
            cursor = &rem[1..];
            tiles.push((x, y));
        }

        Self {
            input,
            tiles,
            idx1: 0usize,
            idx2: 0usize,
            valid: false,
            skip_to_valid: false,
        }
    }
}

fn part_two_tick(state: &mut Part2App<'_>) {
    state.valid = false;
    if state.skip_to_valid {
        while !state.valid {
            part_two_single_check(state);
        }
    } else {
        part_two_single_check(state);
    }
}

fn part_two_single_check(state: &mut Part2App<'_>) {
    state.idx2 += 1;
    if state.idx2 == state.tiles.len() {
        state.idx1 += 1;
        if state.idx1 == state.tiles.len() - 1 {
            state.idx1 = 0;
        }
        state.idx2 = state.idx1 + 1;
    }
    let p1 = state.tiles[state.idx1];
    let p2 = state.tiles[state.idx2];
    let (min_x, max_x) = if p1.0 < p2.0 {
        (p1.0, p2.0)
    } else {
        (p2.0, p1.0)
    };
    let (min_y, max_y) = if p1.1 < p2.1 {
        (p1.1, p2.1)
    } else {
        (p2.1, p1.1)
    };
    if state
        .tiles
        .iter()
        .any(|&(x3, y3)| x3 > min_x && x3 < max_x && y3 > min_y && y3 < max_y)
    {
        return;
    }
    if any_lines_crossing(state.idx1, state.idx2, &state.tiles) {
        return;
    }
    state.valid = true;
}

impl Widget for &Part2App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x = self.tiles[self.idx1].0 as f64;
        let y = self.tiles[self.idx1].1 as f64;
        let x2 = self.tiles[self.idx2].0 as f64;
        let y2 = self.tiles[self.idx2].1 as f64;
        let title = Line::from("Part Two".bold());
        let status = Text::from(format!(
            "P1: ({},{}), P2: ({},{})\nReset: R Pause/Unpause: P, Toggle Skip To Next Valid: V\nCurrently {}",
            x,
            y,
            x2,
            y2,
            if self.skip_to_valid {
                "skipping"
            } else {
                "not skipping"
            }
        ));
        let block = Block::bordered().title(title.centered());
        let [status_area, dial_area] =
            Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(block.inner(area));
        block.render(area, buf);
        Canvas::default()
            .x_bounds([0f64, 100000f64])
            .y_bounds([0f64, 100000f64])
            .block(Block::bordered().border_type(BorderType::Double))
            .paint(|ctx| {
                let mut prev_tile = self.tiles[0];
                for curr_tile in self.tiles[1..].iter() {
                    ctx.draw(&canvas::Line {
                        x1: prev_tile.0 as f64,
                        y1: prev_tile.1 as f64,
                        x2: curr_tile.0 as f64,
                        y2: curr_tile.1 as f64,
                        color: ratatui::style::Color::White,
                    });
                    prev_tile = *curr_tile;
                }
                ctx.draw(&canvas::Line {
                    x1: prev_tile.0 as f64,
                    y1: prev_tile.1 as f64,
                    x2: self.tiles[0].0 as f64,
                    y2: self.tiles[0].1 as f64,
                    color: ratatui::style::Color::White,
                });
                let rect_color = if self.valid {
                    ratatui::style::Color::Green
                } else {
                    ratatui::style::Color::Red
                };
                ctx.draw(&Rectangle {
                    x,
                    y,
                    width: x2 - x,
                    height: y2 - y,
                    color: rect_color,
                });
            })
            .render(dial_area, buf);
        Paragraph::new(status).centered().render(status_area, buf);
    }
}

pub fn part_one_tui(input: &str) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = Part1App::new(input.as_bytes());
    let mut paused = true;
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
    let mut paused = true;
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
                KeyCode::Char('v') => state.skip_to_valid = !state.skip_to_valid,
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
