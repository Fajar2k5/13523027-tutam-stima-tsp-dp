use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use crate::app::{App, Focus};
use crossterm::event::{read, Event, KeyCode, KeyModifiers};

fn draw_ui(app: &App, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) {
    terminal
        .draw(|f| {
            let size = f.size();

            // Split vartikal, atas edit matriks, bawah output
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(size);

            draw_matrix_block(f, chunks[0], app);
            draw_output_block(f, chunks[1], app);
        })
        .unwrap();
}

fn draw_matrix_block<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: ratatui::backend::Backend,
{
    // If parse_error border is red
    let border_style = if app.parse_error.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(Span::styled(
            " Edit Matriks Ketetanggaan (n×n) ",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(border_style);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Display parse error (if any)
    if let Some(err) = &app.parse_error {
        let err_para = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        let err_area = Rect::new(inner.x, inner.y, inner.width, 1);
        f.render_widget(err_para, err_area);
    }

    let y_offset = if app.parse_error.is_some() { 1 } else { 0 };
    for (idx, line) in app.matrix_lines.iter().enumerate() {
        let is_selected = matches!(app.focus, Focus::EditingMatrix) && idx == app.cursor;
        let style = if is_selected {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default()
        };
        let text = Spans::from(Span::styled(line.clone(), style));
        let line_area = Rect::new(
            inner.x,
            inner.y + y_offset as u16 + idx as u16,
            inner.width,
            1,
        );
        f.render_widget(Paragraph::new(text), line_area);
    }

    let button_y = inner.y + y_offset as u16 + app.matrix_lines.len() as u16 + 1;
    let button_style = if matches!(app.focus, Focus::ComputeButton) {
        Style::default()
            .bg(Color::Green)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    };
    let button = Paragraph::new(Span::styled("[ Hitung ]", button_style));
    let button_area = Rect::new(inner.x, button_y, inner.width, 1);
    f.render_widget(button, button_area);
}

fn draw_output_block<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: ratatui::backend::Backend,
{
    let block = Block::default()
        .title(Span::styled(
            " Hasil / Panduan ",
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    if let Some((cost, tour)) = &app.result {
        lines.push(Spans::from(Span::raw(format!("Shortest Tour Cost: {}", cost))));
        lines.push(Spans::from(Span::raw(format!("Tour Sequence: {:?}", tour))));
    } else {
        lines.push(Spans::from(Span::raw(
            "Gunakan panah atas/bawah pada keyboard untuk pindah line.",
        )));
        lines.push(Spans::from(Span::raw(
            "Ketik angka dan spasi untuk membangun matriks.",
        )));
        lines.push(Spans::from(Span::raw(
            "Tekan enter untuk memilih tombol [ Hitung ] lalu enter lagi untuk memulai solver atau tekan Esc untuk kembali mengedit matriks.",
        )));
        lines.push(Spans::from(Span::raw(
            "Tekan Esc (ketika mengedit matriks) atau Ctrl+C untuk keluar dari program.",
        )));
    }
    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

pub fn run_app_with_n(n: usize) -> crossterm::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(n);

    loop {
        draw_ui(&app, &mut terminal);

        if let Event::Key(key) = read()? {
            // Ctrl+C
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            }
            // Esc di edit mode -> keluar dari app
            if matches!(app.focus, Focus::EditingMatrix) && key.code == KeyCode::Esc {
                break;
            }
            
            app.on_key(key);
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

#[allow(dead_code)]
pub fn run_app() -> crossterm::Result<()> {
    run_app_with_n(4)
}
