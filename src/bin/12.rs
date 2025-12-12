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

advent_of_code::solution!(12);

// struct Present {
//     shape: [u64; 3],
//     area: u64,
// }
//
// impl Display for Present {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("Area: {}\n", self.area))?;
//         for line in self.shape {
//             for i in 0..3 {
//                 let c = if (line >> i & 1) == 1 { '#' } else { '.' };
//                 f.write_char(c)?;
//             }
//             f.write_char('\n')?;
//         }
//         Ok(())
//     }
// }
//
// fn parse_present(mut input: &[u8]) -> (Present, &[u8]) {
//     input = &input[3..];
//     let mut area = 0;
//     let shape: [u64; 3] = from_fn(|_| {
//         let line_as_u32 = unsafe { (input.as_ptr() as *const u32).read_unaligned() };
//         let line = unsafe { _pext_u32(line_as_u32, 0b00000000_00000001_00000001_00000001) } as u64;
//         area += line.count_ones() as u64;
//         input = &input[4..];
//         line
//     });
//     (Present { shape, area }, &input[1..])
// }
//
// fn parse_presents(mut input: &[u8]) -> ([Present; 6], &[u8]) {
//     let presents: [Present; 6] = from_fn(|_| {
//         let (present, rem) = parse_present(input);
//         input = rem;
//         present
//     });
//     (presents, input)
// }

// #[derive(Debug)]
// struct Region {
//     width: u8,
//     height: u8,
//     target: [u8; 6],
// }
//
// fn parse_regions(mut input: &[u8]) -> Vec<Region> {
//     let mut regions = vec![];
//     while !input.is_empty() {
//         let (width, rem) = fast_parse::<u8>(input);
//         let (height, rem) = fast_parse::<u8>(&rem[1..]);
//         input = &rem[2..];
//         let target: [u8; 6] = from_fn(|_| {
//             let (num, rem) = fast_parse::<u8>(input);
//             input = &rem[1..];
//             num
//         });
//         regions.push(Region {
//             width,
//             height,
//             target,
//         })
//     }
//     regions
// }

pub fn part_one(input: &str) -> Option<u64> {
    let mut valid = 0u64;
    let mut input = &input.as_bytes()[96..];
    while !input.is_empty() {
        let (width, rem) = fast_parse::<u32>(input);
        let (height, rem) = fast_parse::<u32>(&rem[1..]);
        input = &rem[2..];
        let mut presents_to_fit = 0;
        for _ in 0..6 {
            let (num, rem) = fast_parse::<u32>(input);
            input = &rem[1..];
            presents_to_fit += num;
        }
        if width * height >= presents_to_fit * 9 {
            valid += 1;
        }
    }
    Some(valid)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
