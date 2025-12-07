use std::{io, mem::swap, time::Duration};

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, canvas::Canvas},
};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start_index = width / 2 - 1;
    debug_assert!(input[start_index] == b'S');
    let mut split = 0u64;
    let mut beams = vec![start_index];
    let mut next_beams: Vec<usize> = vec![];
    for layer in input.chunks_exact(width).step_by(2).skip(1) {
        for &beam in beams.iter() {
            if layer[beam] == b'^' {
                split += 1;
                if next_beams.last().is_none_or(|&b| b != beam - 1) {
                    next_beams.push(beam - 1);
                }
                next_beams.push(beam + 1);
            } else if next_beams.last().is_none_or(|&b| b != beam) {
                next_beams.push(beam);
            }
        }
        swap(&mut beams, &mut next_beams);
        next_beams.clear();
    }
    Some(split)
}

// pub fn part_two(input: &str) -> Option<u64> {
//     let input = input.as_bytes();
//     let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
//     let start_index = width / 2 - 1;
//     debug_assert!(input[start_index] == b'S');
//     let jump_size = 2 * width;
//     let mut cache = HashMap::new();
//     Some(beam_splitter(
//         input,
//         start_index + jump_size,
//         jump_size,
//         &mut cache,
//     ))
// }
//
// fn beam_splitter(
//     input: &[u8],
//     index: usize,
//     jump_size: usize,
//     cache: &mut HashMap<usize, u64>,
// ) -> u64 {
//     if let Some(answer) = cache.get(&index) {
//         return *answer;
//     }
//     if index >= input.len() {
//         return 1;
//     }
//     let answer = if input[index] == b'^' {
//         beam_splitter(input, index + jump_size - 1, jump_size, cache)
//             + beam_splitter(input, index + jump_size + 1, jump_size, cache)
//     } else {
//         beam_splitter(input, index + jump_size, jump_size, cache)
//     };
//     cache.insert(index, answer);
//     answer
// }

// pub fn part_two(input: &str) -> Option<u64> {
//     let input = input.as_bytes();
//     let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
//     let start_index = width / 2 - 1;
//     debug_assert!(input[start_index] == b'S');
//     let mut beams = vec![0; width];
//     beams[start_index] = 1;
//     let mut next_beams = vec![0; width];
//     for layer in input.chunks_exact(width).step_by(2).skip(1) {
//         for (col, &content) in layer[..width].iter().enumerate() {
//             let prev_beam = beams[col];
//             if content == b'^' {
//                 next_beams[col - 1] += prev_beam;
//                 next_beams[col + 1] += prev_beam;
//             } else {
//                 next_beams[col] += prev_beam;
//             }
//         }
//         swap(&mut beams, &mut next_beams);
//         next_beams.fill(0);
//     }
//     Some(beams.iter().sum())
// }

#[derive(Copy, Clone)]
struct Beam {
    col: usize,
    particles: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start_index = width / 2 - 1;
    debug_assert!(input[start_index] == b'S');
    let mut beams = vec![Beam {
        col: start_index,
        particles: 1u64,
    }];
    let mut next_beams: Vec<Beam> = vec![];
    for layer in input.chunks_exact(width).step_by(2).skip(1) {
        for &beam in beams.iter() {
            if layer[beam.col] == b'^' {
                // beam splits
                if let Some(last_beam) = next_beams.last_mut()
                    && last_beam.col == beam.col - 1
                {
                    // there is a previous beam at that column
                    last_beam.particles += beam.particles;
                } else {
                    next_beams.push(Beam {
                        col: beam.col - 1,
                        particles: beam.particles,
                    });
                }
                // right split can always be added
                next_beams.push(Beam {
                    col: beam.col + 1,
                    particles: beam.particles,
                });
            } else if let Some(last_beam) = next_beams.last_mut() {
                // no splitter
                if last_beam.col == beam.col {
                    last_beam.particles += beam.particles;
                } else {
                    next_beams.push(Beam {
                        col: beam.col,
                        particles: beam.particles,
                    });
                }
            }
        }
        swap(&mut beams, &mut next_beams);
        next_beams.clear();
    }
    Some(beams.iter().map(|b| b.particles).sum())
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
