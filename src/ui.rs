use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::*;

pub fn ui(f: &mut Frame, app: &App, form: &mut TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Min(3),
        ])
        .split(f.size());

    // Title bar
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(
        Line::from("Command Line Blackjack")
            .fg(Color::Blue)
            .centered(),
    )
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // Player block
    let player_area = centered_rect(50, 75, chunks[2]);
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    // Dealer block
    let dealer_area = centered_rect(50, 75, chunks[1]);
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    match app.state {
        GameState::EnterBet => {
            let bet_area = centered_rect(50, 25, player_area);
            let bet_form = form.widget();
            f.render_widget(player_block, player_area);
            f.render_widget(dealer_block, dealer_area);
            f.render_widget(bet_form, bet_area);
        }
        GameState::PlayerTurn => {
            let upcard = Paragraph::new(format!(
                "{}\n[HIDDEN CARD]",
                app.dealer_hand.first().unwrap()
            ))
            .block(dealer_block);
            f.render_widget(upcard, dealer_area);
            let mut player_cards: Vec<Line> = Vec::new();
            for card in &app.player_hand {
                player_cards.push(Line::from(format!("{card}\n")));
            }
            player_cards.push(Line::from(format!("Score: {}", app.player_score())));
            let player_view = Paragraph::new(player_cards).block(player_block);
            f.render_widget(player_view, player_area);
        }
        // TODO: Implement UI for winning/losing
        GameState::Win => todo!(),
        GameState::Lose => todo!(),
    }

    // Footer with allowed commands
    let current_keys_hint = {
        match app.state {
            GameState::EnterBet => {
                Span::styled("<Enter> to place bet / <Escape> to quit game", Style::default())
            }
            GameState::PlayerTurn => Span::styled(
                "<h> to hit / <s> to stand / <q> to quit game",
                Style::default(),
            ),
            GameState::Win => {
                Span::styled("<Enter> to play again / <q> to quit", Style::default())
            }
            GameState::Lose => {
                Span::styled("<Enter> to play again / <q> to quit", Style::default())
            }
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint).centered().italic()).block(
        Block::default()
    );

    f.render_widget(key_notes_footer, chunks[3]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
