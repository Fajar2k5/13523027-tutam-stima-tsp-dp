use crossterm::event::{KeyCode, KeyEvent};
use std::num::ParseIntError;

pub enum Focus {
    EditingMatrix,
    ComputeButton,
}

pub struct App {
    pub matrix_lines: Vec<String>,
    pub matrix: Option<Vec<Vec<usize>>>,
    pub result: Option<(usize, Vec<usize>)>,
    pub cursor: usize,
    pub focus: Focus,
    pub parse_error: Option<String>,
}

impl App {
    pub fn new(n: usize) -> Self {
        // initialize awalnya nol semua
        let mut lines = Vec::new();
        for _ in 0..n {
            lines.push("0 ".repeat(n).trim_end().to_string());
        }
        Self {
            matrix_lines: lines,
            matrix: None,
            result: None,
            cursor: 0,
            focus: Focus::EditingMatrix,
            parse_error: None,
        }
    }

    fn parse_matrix(&mut self) -> Result<Vec<Vec<usize>>, String> {
        let n = self.matrix_lines.len();
        let mut matrix = Vec::new();
        for (i, line) in self.matrix_lines.iter().enumerate() {
            let row: Result<Vec<usize>, ParseIntError> = line
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect();
            match row {
                Ok(r) if r.len() == n => matrix.push(r),
                Ok(r) => {
                    return Err(format!(
                        "Line {}: expected {} entries but found {}",
                        i + 1,
                        n,
                        r.len()
                    ))
                }
                Err(e) => return Err(format!("Parsing error on line {}: {}", i + 1, e)),
            }
        }
        Ok(matrix)
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::EditingMatrix => match key.code {
                KeyCode::Up => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.cursor + 1 < self.matrix_lines.len() {
                        self.cursor += 1;
                    }
                }
                KeyCode::Char(c) => {
                    self.matrix_lines[self.cursor].push(c);
                }
                KeyCode::Backspace => {
                    self.matrix_lines[self.cursor].pop();
                }
                KeyCode::Enter => {
                    self.focus = Focus::ComputeButton;
                    self.parse_error = None;
                }
                KeyCode::Esc => {
                    self.parse_error = None;
                    self.matrix = None;
                    self.result = None;
                }
                _ => {}
            },
            Focus::ComputeButton => match key.code {
                KeyCode::Enter => {
                    match self.parse_matrix() {
                        Ok(m) => {
                            self.matrix = Some(m.clone());
                            let (cost, tour) = crate::tsp_solver::solve_tsp(m);
                            self.result = Some((cost, tour));
                            self.parse_error = None;
                        }
                        Err(msg) => {
                            self.parse_error = Some(msg);
                        }
                    }
                }
                KeyCode::Esc => {
                    self.focus = Focus::EditingMatrix;
                    self.parse_error = None;
                }
                _ => {}
            },
        }
    }
}
