use chrono::Datelike;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::{
    app::{App, DayCell},
    lunar,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.size());

    frame.render_widget(header(app), chunks[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    frame.render_widget(calendar(app), body[0]);
    frame.render_widget(details(app), body[1]);
    frame.render_widget(help_bar(), chunks[2]);
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
    .block(Block::default().borders(Borders::ALL).title("RiLi"))
}

fn calendar(app: &App) -> Table<'_> {
    let headers = ["一", "二", "三", "四", "五", "六", "日"]
        .into_iter()
        .map(|label| {
            Cell::from(label).style(
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

    Table::new(rows, widths)
        .header(header_row)
        .block(Block::default().title("月历").borders(Borders::ALL))
}

fn day_cell(cell: DayCell) -> Cell<'static> {
    let label = cell
        .holiday
        .map(|info| info.name.to_string())
        .or_else(|| cell.solar_term.map(|name| name.to_string()))
        .or_else(|| cell.lunar.map(|info| info.display_label().to_string()))
        .unwrap_or_else(|| "--".to_string());
    let has_label = cell.holiday.is_some() || cell.solar_term.is_some() || cell.lunar.is_some();
    let text = format!(
        "{:>2}{}{}",
        cell.date.day(),
        if has_label { "\n" } else { "" },
        label
    );
    let mut style = if cell.is_current_month {
        Style::default()
    } else {
        Style::default().fg(Color::DarkGray)
    };
    if cell.is_selected {
        style = style
            .bg(Color::Blue)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);
    } else if cell.is_today {
        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
    }
    Cell::from(text).style(style)
}

fn details(app: &App) -> Paragraph<'_> {
    let selected = app.selected_date();
    let holiday = app.selected_holiday();
    let mut current_line = format!(
        "当前：{} ({:?})",
        selected.format("%Y-%m-%d"),
        selected.weekday()
    );
    if let Some(info) = holiday {
        current_line.push_str(&format!(" · {}", info.name));
    }
    let mut lines = vec![Line::from(current_line)];
    let term_line = app
        .selected_solar_term()
        .map(|name| name.to_string())
        .unwrap_or_else(|| "-".to_string());
    lines.push(Line::from(format!("节气：{}", term_line)));
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

    Paragraph::new(lines).block(Block::default().title("详情").borders(Borders::ALL))
}

fn help_bar() -> Paragraph<'static> {
    Paragraph::new("←/→ 切换月份 · ↑/↓ 切换年份 · h/j/k/l 移动日期 · t 回到今天 · q 退出")
        .block(Block::default().borders(Borders::ALL).title("快捷键"))
}
