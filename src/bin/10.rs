use std::{io, iter::from_fn, time::Duration};

use advent_of_code::util::fast_parse;
use good_lp::*;
use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, canvas::Canvas},
};

advent_of_code::solution!(10);

fn parse_light_requirement(mut input: &[u8]) -> (u32, &[u8]) {
    debug_assert!(input[0] == b'[');
    input = &input[1..];
    let mut target = 0u32;
    let mut lights = 0;
    while input[0] != b']' {
        lights += 1;
        target >>= 1;
        if input[0] == b'#' {
            target |= 1 << 31;
        }
        input = &input[1..];
    }
    (target >> (32 - lights), &input[1..])
}

fn parse_button_for_lights(mut input: &[u8]) -> (u32, &[u8]) {
    debug_assert!(input[0] == b'(');
    input = &input[1..];
    let mut button = 0u32;
    while input[0] != b' ' {
        let (num, rem) = fast_parse::<u8>(input);
        button |= 1 << num;
        input = &rem[1..];
    }
    (button, input)
}

fn find_minimum_presses_for_lights(target: u32, buttons: &[u32]) -> u64 {
    if target == 0 {
        return 0;
    }
    if buttons.is_empty() {
        return u64::MAX / 2;
    }
    (find_minimum_presses_for_lights(target ^ buttons[0], &buttons[1..]) + 1)
        .min(find_minimum_presses_for_lights(target, &buttons[1..]))
}

fn parse_button_for_joltage(mut input: &[u8]) -> (Vec<u32>, &[u8]) {
    debug_assert!(input[0] == b'(');
    input = &input[1..];
    let mut button = vec![];
    while input[0] != b' ' {
        let (num, rem) = fast_parse::<u32>(input);
        button.push(num);
        input = &rem[1..];
    }
    (button, input)
}
fn find_minimum_presses_for_joltage(buttons: &[Vec<u32>], joltage_reqs: &[u32]) -> u64 {
    let mut vars = ProblemVariables::new();
    let button_variable_defs = vec![variable().integer().min(0); buttons.len()];
    let button_variables: Vec<Variable> = vars.add_all(button_variable_defs);
    let to_minimize: Expression = button_variables.iter().sum();
    let mut problem = vars.minimise(to_minimize).using(default_solver);
    for (req_index, &req) in joltage_reqs.iter().enumerate() {
        let req_index = req_index as u32;
        let req_equation: Expression = buttons
            .iter()
            .zip(&button_variables)
            .filter_map(|(b, bv)| {
                if b.contains(&req_index) {
                    Some(bv)
                } else {
                    None
                }
            })
            .sum();
        problem = problem.with(req_equation.eq(req));
    }
    problem.set_parameter("log", "0");
    let solution = problem.solve().unwrap();
    button_variables
        .iter()
        .map(|&bv| solution.value(bv).round() as u64)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    Some(
        from_fn(|| {
            if input.is_empty() {
                return None;
            }
            // println!("{}", str::from_utf8(input).unwrap());
            let (target, rem) = parse_light_requirement(input);
            let mut buttons = vec![];
            input = rem;
            while input[1] != b'{' {
                let (button, rem) = parse_button_for_lights(&input[1..]);
                buttons.push(button);
                input = rem;
            }
            let next_line_start = input.iter().position(|&c| c == b'\n').unwrap();
            input = &input[next_line_start + 1..];
            Some((target, buttons))
        })
        .map(|(target, buttons)| find_minimum_presses_for_lights(target, &buttons))
        .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = input.as_bytes();
    Some(
        from_fn(|| {
            if input.is_empty() {
                return None;
            }
            // println!("{}", str::from_utf8(input).unwrap());
            let first_button_start = input.iter().position(|&c| c == b' ').unwrap();
            input = &input[first_button_start..];
            let mut buttons = vec![];
            while input[1] != b'{' {
                let (button, rem) = parse_button_for_joltage(&input[1..]);
                buttons.push(button);
                input = rem;
            }
            input = &input[2..];
            let mut joltage_reqs = Vec::with_capacity(32);
            while input[0] != b'\n' {
                let (num, rem) = fast_parse(input);
                joltage_reqs.push(num);
                input = &rem[1..];
            }

            input = &input[1..];
            Some((buttons, joltage_reqs))
        })
        .map(|(buttons, joltage_reqs)| find_minimum_presses_for_joltage(&buttons, &joltage_reqs))
        .sum(),
    )
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
