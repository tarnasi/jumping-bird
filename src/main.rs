use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::io;
use std::time::{Duration, Instant};

const GAME_WIDTH: u16 = 80;
const GAME_HEIGHT: u16 = 20;
const BIRD_START_X: u16 = 5;
const BIRD_START_Y: u16 = GAME_HEIGHT / 2;
const MAX_JUMPS: u8 = 3;
const JUMP_POWER: i16 = 4;
const GRAVITY: f32 = 0.2;
const OBSTACLE_SPEED: u16 = 1;
const SPAWN_RATE: usize = 15; // spawn obstacle every 15 frames

#[derive(Clone, Copy, Debug)]
struct Bird {
    x: u16,
    y: u16,
    velocity: f32,
    jumps_left: u8,
    is_jumping: bool,
}

impl Bird {
    fn new() -> Self {
        Self {
            x: BIRD_START_X,
            y: BIRD_START_Y,
            velocity: 0.0,
            jumps_left: MAX_JUMPS,
            is_jumping: false,
        }
    }

    fn jump(&mut self) {
        if self.jumps_left > 0 {
            self.velocity = -(JUMP_POWER as f32);
            self.jumps_left -= 1;
            self.is_jumping = true;
        }
    }

    fn update(&mut self) {
        // Apply gravity
        self.velocity += GRAVITY;
        self.y = ((self.y as f32) + self.velocity) as u16;

        // Clamp to ground level
        if self.y >= GAME_HEIGHT - 1 {
            self.y = GAME_HEIGHT - 1;
            self.velocity = 0.0;
            self.is_jumping = false;
            self.jumps_left = MAX_JUMPS; // Reset jumps when landing
        }

        // Clamp to ceiling
        if self.y == 0 {
            self.velocity = 0.0;
        }
    }
}

#[derive(Clone, Debug)]
struct Obstacle {
    x: u16,
    y: u16,
    width: u16,
    obstacle_type: ObstacleType,
}

#[derive(Clone, Debug, PartialEq)]
enum ObstacleType {
    Tree,
    Rock,
    Cloud,
}

impl Obstacle {
    fn new(x: u16, y: u16, obstacle_type: ObstacleType) -> Self {
        let width = match obstacle_type {
            ObstacleType::Tree => 3,
            ObstacleType::Rock => 2,
            ObstacleType::Cloud => 4,
        };
        Self {
            x,
            y,
            width,
            obstacle_type,
        }
    }

    fn update(&mut self) {
        if self.x > 0 {
            self.x -= OBSTACLE_SPEED;
        }
    }

    fn is_off_screen(&self) -> bool {
        self.x + self.width < 1
    }

    fn collides_with_bird(&self, bird: &Bird) -> bool {
        let bird_left = bird.x;
        let bird_right = bird.x + 1;
        let bird_top = bird.y;
        let bird_bottom = bird.y + 1;

        let obstacle_left = self.x;
        let obstacle_right = self.x + self.width;
        let obstacle_top = self.y;
        let obstacle_bottom = self.y + 1;

        bird_right > obstacle_left
            && bird_left < obstacle_right
            && bird_bottom > obstacle_top
            && bird_top < obstacle_bottom
    }
}

#[derive(Debug)]
struct Game {
    bird: Bird,
    obstacles: Vec<Obstacle>,
    score: u32,
    game_over: bool,
    frame_count: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            bird: Bird::new(),
            obstacles: Vec::new(),
            score: 0,
            game_over: false,
            frame_count: 0,
        }
    }

    fn handle_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(' ') => self.bird.jump(),
            KeyCode::Char('q') | KeyCode::Esc => self.game_over = true,
            _ => {}
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.bird.update();

        // Update obstacles
        for obstacle in &mut self.obstacles {
            obstacle.update();
        }

        // Remove off-screen obstacles
        self.obstacles.retain(|o| !o.is_off_screen());

        // Spawn new obstacles
        if self.frame_count % SPAWN_RATE == 0 {
            let obstacle_type = match self.frame_count / SPAWN_RATE % 3 {
                0 => ObstacleType::Tree,
                1 => ObstacleType::Rock,
                _ => ObstacleType::Cloud,
            };
            let y = if obstacle_type == ObstacleType::Cloud {
                3
            } else {
                GAME_HEIGHT - 2
            };
            self.obstacles
                .push(Obstacle::new(GAME_WIDTH, y, obstacle_type));
        }

        // Collision detection
        for obstacle in &self.obstacles {
            if obstacle.collides_with_bird(&self.bird) {
                self.game_over = true;
            }
        }

        // Increase score
        self.score += 1;
        self.frame_count += 1;
    }
}

fn render(frame: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(GAME_HEIGHT as u16 + 2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.area());

    // Title and score
    let title = vec![
        Line::from(vec![
            Span::styled("🐦 JUMPING BIRD ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(format!("| Score: {} | Jumps Left: {}", game.score / 10, game.bird.jumps_left)),
        ]),
    ];
    let title_widget = Paragraph::new(title).alignment(Alignment::Center);
    frame.render_widget(title_widget, chunks[0]);

    // Game area
    let game_block = Block::default()
        .borders(Borders::ALL)
        .title(" Game Area ")
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(game_block, chunks[1]);

    let game_area = Rect {
        x: chunks[1].x + 1,
        y: chunks[1].y + 1,
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
    };

    // Render game content
    render_game_area(frame, game, game_area);

    // Instructions/Status
    let status_text = if game.game_over {
        vec![
            Line::from(vec![Span::styled(
                "GAME OVER!",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(format!("Final Score: {} | Press Q to quit", game.score / 10)),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Space", Style::default().fg(Color::Green)),
                Span::raw(": Jump | "),
                Span::styled("Q", Style::default().fg(Color::Green)),
                Span::raw(": Quit"),
            ]),
            Line::from(format!(
                "Avoid trees, rocks, and clouds! Survive as long as possible."
            )),
        ]
    };
    let status_widget = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(status_widget, chunks[2]);
}

fn render_game_area(frame: &mut Frame, game: &Game, game_area: Rect) {
    // Create a buffer for the game area
    let mut game_buffer: Vec<Vec<char>> =
        vec![vec![' '; GAME_WIDTH as usize]; GAME_HEIGHT as usize];

    // Draw ground
    for x in 0..GAME_WIDTH {
        game_buffer[(GAME_HEIGHT - 1) as usize][x as usize] = '=';
    }

    // Draw bird
    if (game.bird.y as usize) < GAME_HEIGHT as usize {
        game_buffer[game.bird.y as usize][game.bird.x as usize] = '🐤';
    }

    // Draw obstacles
    for obstacle in &game.obstacles {
        let symbol = match obstacle.obstacle_type {
            ObstacleType::Tree => '🌲',
            ObstacleType::Rock => '◆',
            ObstacleType::Cloud => '☁',
        };

        if (obstacle.y as usize) < GAME_HEIGHT as usize && (obstacle.x as usize) < GAME_WIDTH as usize {
            for i in 0..obstacle.width {
                let ox = (obstacle.x + i) as usize;
                if ox < GAME_WIDTH as usize {
                    game_buffer[obstacle.y as usize][ox] = symbol;
                }
            }
        }
    }

    // Convert buffer to Line's for rendering
    let mut lines = Vec::new();
    for row in game_buffer {
        let line_str: String = row.iter().collect();
        lines.push(Line::from(Span::raw(line_str)));
    }

    let game_content = Paragraph::new(lines);
    frame.render_widget(game_content, game_area);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| render(frame, &game))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                game.handle_input(key.code);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            game.update();
            last_tick = Instant::now();
        }

        if game.game_over {
            // Wait for quit
            loop {
                if let Event::Key(key) = event::read()? {
                    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        return Ok(());
                    }
                }
            }
        }
    }
}
