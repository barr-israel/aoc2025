use std::{
    collections::{BinaryHeap, HashSet},
    io,
    mem::swap,
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

advent_of_code::solution!(8);

fn parse_input(mut input: &[u8]) -> Vec<(i32, i32, i32)> {
    let mut boxes = vec![];
    while !input.is_empty() {
        let (x, rem) = fast_parse(input);
        let (y, rem) = fast_parse(&rem[1..]);
        let (z, rem) = fast_parse(&rem[1..]);
        boxes.push((x, y, z));
        input = &rem[1..];
    }
    boxes
}

fn calc_distance(box1: (i32, i32, i32), box2: (i32, i32, i32)) -> u64 {
    let dx = (box1.0 - box2.0) as i64;
    let dy = (box1.1 - box2.1) as i64;
    let dz = (box1.2 - box2.2) as i64;
    (dx * dx + dy * dy + dz * dz).isqrt() as u64
}

#[derive(Eq, PartialEq, Debug)]
struct Distance {
    box1: u32,
    box2: u32,
    distance: u64,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn boxes_to_distances(boxes: &[(i32, i32, i32)]) -> BinaryHeap<Distance> {
    let distance_count = (boxes.len() * (boxes.len() + 1)) / 2;
    let mut distances = BinaryHeap::with_capacity(distance_count);
    for (idx1, box1) in boxes.iter().enumerate() {
        for (idx2, box2) in boxes[idx1 + 1..].iter().enumerate() {
            // correct the offset
            let idx2 = idx2 + idx1 + 1;
            distances.push(Distance {
                box1: idx1 as u32,
                box2: idx2 as u32,
                distance: calc_distance(*box1, *box2),
            });
        }
    }
    distances
}

fn mul_largest_three(circuits: Vec<HashSet<u32>>) -> u64 {
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    circuits.into_iter().map(|c| c.len()).for_each(|size| {
        if size > max1 {
            max3 = max2;
            max2 = max1;
            max1 = size;
        } else if size > max2 {
            max3 = max2;
            max2 = size;
        } else if size > max3 {
            max3 = size;
        }
    });
    (max1 * max2 * max3) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_inner(input, 1000)
}

fn part_one_inner(input: &str, to_connect: u32) -> Option<u64> {
    let boxes = parse_input(input.as_bytes());
    let box_count = boxes.len();
    let mut distances = boxes_to_distances(&boxes);
    let mut box_to_circuit: Vec<u32> = (0..box_count as u32).collect();
    let mut circuits: Vec<_> = (0..box_count as u32)
        .map(|b| {
            let mut s = HashSet::new();
            s.insert(b);
            s
        })
        .collect();
    for _ in 0..to_connect {
        let potential_connection = distances
            .pop()
            .expect("There must be at least 1000 possible connections");
        let mut circuit1_id = box_to_circuit[potential_connection.box1 as usize];
        let mut circuit2_id = box_to_circuit[potential_connection.box2 as usize];
        if circuit1_id != circuit2_id {
            if circuit2_id < circuit1_id {
                // make id 1 the smallest for the split
                swap(&mut circuit1_id, &mut circuit2_id);
            }
            let (s1, s2) = circuits.split_at_mut(circuit2_id as usize);
            let mut circuit1 = &mut s1[circuit1_id as usize];
            let mut circuit2 = &mut s2[0];
            if circuit2.len() < circuit1.len() {
                // better to combine smallest into largest, so swap them
                swap(&mut circuit1, &mut circuit2);
                swap(&mut circuit1_id, &mut circuit2_id);
            }
            circuit1.drain().for_each(|box_id| {
                box_to_circuit[box_id as usize] = circuit2_id;
                circuit2.insert(box_id);
            });
        }
    }
    Some(mul_largest_three(circuits))
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_input(input.as_bytes());
    let box_count = boxes.len();
    let mut distances = boxes_to_distances(&boxes);
    let mut box_to_circuit: Vec<u32> = (0..box_count as u32).collect();
    let mut circuits: Vec<_> = (0..box_count as u32)
        .map(|b| {
            let mut s = HashSet::new();
            s.insert(b);
            s
        })
        .collect();
    loop {
        let potential_connection = distances
            .pop()
            .expect("There must be at least 1000 possible connections");
        let mut circuit1_id = box_to_circuit[potential_connection.box1 as usize];
        let mut circuit2_id = box_to_circuit[potential_connection.box2 as usize];
        if circuit1_id != circuit2_id {
            if circuit2_id < circuit1_id {
                // make id 1 the smallest for the split
                swap(&mut circuit1_id, &mut circuit2_id);
            }
            let (s1, s2) = circuits.split_at_mut(circuit2_id as usize);
            let mut circuit1 = &mut s1[circuit1_id as usize];
            let mut circuit2 = &mut s2[0];
            if circuit2.len() < circuit1.len() {
                // better to combine smallest into largest, so swap them
                swap(&mut circuit1, &mut circuit2);
                swap(&mut circuit1_id, &mut circuit2_id);
            }
            circuit1.drain().for_each(|box_id| {
                box_to_circuit[box_id as usize] = circuit2_id;
                circuit2.insert(box_id);
            });
            if circuit2.len() == box_count {
                return Some(
                    boxes[potential_connection.box1 as usize].0 as u64
                        * boxes[potential_connection.box2 as usize].0 as u64,
                );
            }
        }
    }
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
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
