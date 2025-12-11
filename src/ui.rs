use chrono::Datelike;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
};

use crate::{
    app::{App, DayCell, JumpPromptView},
    config::{Action, KeyBindings},
    lunar,
};

/// Main entry point for rendering the UI
pub fn draw(frame: &mut Frame, app: &App, bindings: &KeyBindings) {
    let (help_widget, help_height) = help_bar(bindings);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(help_height),
        ])
        .split(frame.size());

    frame.render_widget(header(app), chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    frame.render_widget(calendar(app), body[0]);
    frame.render_widget(details(app), body[1]);
    frame.render_widget(help_widget, chunks[2]);
    // Render the jump prompt overlay
    if let Some(prompt) = app.jump_prompt_view() {
        draw_jump_prompt(frame, prompt);
    }
}

fn header(app: &App) -> Paragraph<'_> {
    let solar = format!("公历：{} 年 {:02} 月", app.view_year(), app.view_month());
    let today_text = format!("今天：{}", app.today().format("%Y-%m-%d"));
    let lunar_text = if let Some(info) = app.month_anchor_lunar() {
        let gz = lunar::gan_zhi_year(info.date.year);
        let zodiac = lunar::zodiac_animal(info.date.year);
        format!("农历：{}年 · {}年", gz, zodiac)
    } else {
        "农历：暂不可用".to_string()
    };
    Paragraph::new(Line::from(Span::raw(format!(
        "{} | {} | {}",
        solar, today_text, lunar_text
    ))))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .title("MoLi"),
    )
}

/// Calendar display
fn calendar(app: &App) -> Table<'_> {
    let headers = ["一", "二", "三", "四", "五", "六", "日"]
        .into_iter()
        .map(|label| {
            let line = Line::from(label).alignment(Alignment::Center);
            Cell::from(line).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header_row = Row::new(headers).height(1);
    let widths = [Constraint::Ratio(1, 7); 7];

    let rows = app
        .month_rows()
        .into_iter()
        .map(|week| {
            let cells = week.into_iter().map(|cell| day_cell(cell));
            Row::new(cells).height(3)
        })
        .collect::<Vec<_>>();

    Table::new(rows, widths).header(header_row).block(
        Block::default()
            .border_type(BorderType::Rounded)
            .title("月历")
            .borders(Borders::ALL),
    )
}

fn day_cell(cell: DayCell) -> Cell<'static> {
    // Whether to show holiday/solar-term/lunar labels next to the date number
    let has_label = cell.holiday.is_some() || cell.solar_term.is_some() || cell.lunar.is_some();
    let mut lines =
        vec![Line::from(format!("{:02}", cell.date.day())).alignment(Alignment::Center)];
    if has_label {
        // Label priority: holiday > solar term > lunar date
        let label = cell
            .holiday
            .map(|info| info.name.to_string())
            .or_else(|| cell.solar_term.map(|name| name.to_string()))
            .or_else(|| cell.lunar.map(|info| info.display_label().to_string()))
            .unwrap_or_else(|| "--".to_string());
        lines.push(Line::from(label).alignment(Alignment::Center));
    }
    let mut style = if cell.is_current_month {
        Style::default()
    } else {
        Style::default().fg(Color::DarkGray)
    };
    if cell.is_selected {
        style = style
            .bg(Color::Green)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);
    } else if cell.is_today {
        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
    }
    Cell::from(lines).style(style)
}

/// Selected date detail panel
fn details(app: &App) -> Paragraph<'_> {
    let selected = app.selected_date();
    let holiday = app.selected_holiday();
    let holiday_suffix = holiday
        .map(|info| format!(" · {}", info.name))
        .unwrap_or_default();
    let current_line = format!(
        "当前：{} ({:?}){}",
        selected.format("%Y-%m-%d"),
        selected.weekday(),
        holiday_suffix,
    );
    let mut lines = vec![Line::from(current_line)];
    let term_line = app
        .selected_solar_term()
        .map(|name| format!("节气：{}", name))
        .unwrap_or_else(|| "节气: -".to_string());
    lines.push(Line::from(term_line));
    if let Some(info) = holiday {
        lines.push(Line::from(format!(
            "{}：{} - {}",
            info.category.label(),
            info.name,
            info.note
        )));
    }

    if let Some(info) = app.selected_lunar() {
        let gz = lunar::gan_zhi_year(info.date.year);
        let zodiac = lunar::zodiac_animal(info.date.year);
        let lunar_line = format!(
            "农历：{}年 {} {}",
            gz,
            info.month_label(),
            info.display_label()
        );
        lines.push(Line::from(lunar_line));
        lines.push(Line::from(format!("生肖：{}", zodiac)));
        let festival_text = info.festival.unwrap_or("-");
        lines.push(Line::from(format!("节日：{}", festival_text)));
    } else {
        lines.push(Line::from("农历：超出支持范围"));
    }

    Paragraph::new(lines)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .title("详情")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true })
}

fn help_bar(bindings: &KeyBindings) -> (Paragraph<'static>, u16) {
    let prev_month = format_actions(bindings, Action::PrevMonth);
    let next_month = format_actions(bindings, Action::NextMonth);
    let prev_year = format_actions(bindings, Action::PrevYear);
    let next_year = format_actions(bindings, Action::NextYear);
    let move_left = format_actions(bindings, Action::MoveLeft);
    let move_right = format_actions(bindings, Action::MoveRight);
    let move_up = format_actions(bindings, Action::MoveUp);
    let move_down = format_actions(bindings, Action::MoveDown);
    let back_today = format_actions(bindings, Action::BackToToday);
    let quit = format_actions(bindings, Action::Quit);
    let jump_to = format_actions(bindings, Action::OpenJumpPrompt);
    let lines = vec![
        Line::from(format!(
            "左:{} 右:{} 上:{} 下:{} · {} / {} 切换月份 · {} / {} 切换年份",
            move_left, move_right, move_up, move_down, prev_month, next_month, prev_year, next_year
        )),
        Line::from(format!(
            "{} 回到今天 · {} 跳转日期 · {} 退出 · 配置：~/.config/moli/key_bindings.ron",
            back_today, jump_to, quit
        )),
    ];
    let height = lines.len() as u16 + 2;
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .title("快捷键"),
        )
        .wrap(Wrap { trim: true });
    (paragraph, height)
}

fn format_actions(bindings: &KeyBindings, action: Action) -> String {
    let labels = bindings.labels_for(action);
    if labels.is_empty() {
        "未绑定".into()
    } else {
        labels.join("/")
    }
}

fn draw_jump_prompt(frame: &mut Frame, prompt: JumpPromptView<'_>) {
    // Center a 40x15 window on the screen
    let area = centered_rect(40, 15, frame.size());
    // Clear the window area
    frame.render_widget(Clear, area);
    // Build prompt lines
    let mut lines = vec![
        Line::from(format!("目标日期 (YYYY-MM-DD)：{}", prompt.input)).alignment(Alignment::Left),
        Line::from("Enter 确认 · Esc 取消").style(Style::default().fg(Color::Gray)),
    ];
    if let Some(err) = prompt.error {
        lines.push(Line::from(err).style(Style::default().fg(Color::Red)));
    }
    // Build the paragraph widget
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .border_type(BorderType::Rounded)
                .title("跳转到指定日期")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });
    // Render the paragraph
    frame.render_widget(paragraph, area);
}

/// Split horizontally into three parts with ratios (100 - percent_x)/2 : percent_x : (100 - percent_x)/2
/// Take the middle part and split it vertically with ratios (100 - percent_y)/2 : percent_y : (100 - percent_y)/2
/// Return the centered area from that middle block
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(horizontal[1])[1]
}
