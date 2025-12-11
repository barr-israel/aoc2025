use std::{arch::x86_64::_pext_u32, io, time::Duration};

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, canvas::Canvas},
};

advent_of_code::solution!(11);

fn read_label_code(input: &[u8]) -> u32 {
    let raw_code = unsafe { (input.as_ptr() as *const u32).read_unaligned() };
    unsafe { _pext_u32(raw_code, 0b00000000_00011111_00011111_00011111) }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut connections = vec![Vec::<u32>::new(); 1 << 15];
    let mut cache = vec![None; 1 << 15];
    while !input.is_empty() {
        let label_code = read_label_code(input) as usize;
        input = &input[4..];
        while input[0] == b' ' {
            let connected_to = read_label_code(&input[1..]);
            connections[label_code].push(connected_to);
            // skip to next connection
            input = &input[4..];
        }
        // skip \n
        input = &input[1..];
    }
    Some(explore_connections(
        read_label_code(b"you "),
        &connections,
        &mut cache,
    ))
}

fn explore_connections(
    current_position: u32,
    connections: &[Vec<u32>],
    cache: &mut [Option<u64>],
) -> u64 {
    const OUT: u32 = 21167; // cant call read_label_code in const so its hard coded like this
    if current_position == OUT {
        return 1;
    }
    if let Some(paths) = cache[current_position as usize] {
        return paths;
    }
    let paths = connections[current_position as usize]
        .iter()
        .map(|c| explore_connections(*c, connections, cache))
        .sum();
    cache[current_position as usize] = Some(paths);
    paths
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    let mut connections = vec![Vec::<u32>::new(); 1 << 15];
    let mut cache = vec![None; 1 << 17];
    while !input.is_empty() {
        let label_code = read_label_code(input) as usize;
        input = &input[4..];
        while input[0] == b' ' {
            let connected_to = read_label_code(&input[1..]);
            connections[label_code].push(connected_to);
            // skip to next connection
            input = &input[4..];
        }
        // skip \n
        input = &input[1..];
    }
    Some(explore_connections_part2(
        read_label_code(b"svr "),
        &connections,
        &mut cache,
        false,
        false,
    ))
}

fn explore_connections_part2(
    current_position: u32,
    connections: &[Vec<u32>],
    cache: &mut [Option<u64>],
    mut passed_fft: bool,
    mut passed_dac: bool,
) -> u64 {
    // cant call read_label_code in const so its hard coded like this
    const OUT: u32 = 21167;
    const FFT: u32 = 20678;
    const DAC: u32 = 3108;
    if current_position == OUT {
        return (passed_fft & passed_dac) as u64;
    }
    let cache_index =
        ((passed_fft as usize) << 16) | ((passed_dac as usize) << 15) | (current_position as usize);
    if let Some(paths) = cache[cache_index] {
        return paths;
    }
    if current_position == FFT {
        passed_fft = true;
    }
    if current_position == DAC {
        passed_dac = true;
    }

    let paths = connections[current_position as usize]
        .iter()
        .map(|c| explore_connections_part2(*c, connections, cache, passed_fft, passed_dac))
        .sum();
    cache[cache_index] = Some(paths);
    paths
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
