use tetra::graphics::{self, Color, Rectangle, Texture, Text, Font};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    score: Stats,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
            );

        
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player1_position = Vec2::new(
            16.0, 
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);
        let player2_position = Vec2::new(
            WINDOW_WIDTH - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0);
        Ok(GameState { 
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            score: Stats::new(Some(Vec2::new(0,0))),
        })
    }
    fn restart(&mut self, ctx: &mut Context) {

        let ball_texture = Texture::new(ctx, "./resources/ball.png").unwrap();
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
            );
        self.ball = Entity::with_velocity(ball_texture, ball_position, ball_velocity);
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(&self.ball.position.x / WINDOW_WIDTH, 0.5, 0.3));
        graphics::draw(ctx, &self.ball.texture, self.ball.position);
        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.score.content_label, self.score.position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::E) {
            self.player1.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::O) {
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::P) {
            self.player2.position.y += PADDLE_SPEED;
        }
        self.ball.position += self.ball.velocity;
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();
        let paddle_hit  = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };
        if let Some(paddle) = paddle_hit {
            // Increase the ball's velocity, then flip it.
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            //Calc. the offset between the paddle and the ball, as a number between
            // -1 and 1
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }
        if self.ball.position.x > WINDOW_WIDTH {
            
            &self.score.update(Vec2::new(1,0));
            &self.restart(ctx);
            println!("Player 1 wins!");
        }
        if self.ball.position.x < 0.0 {
            &self.score.update(Vec2::new(0,1));
            &self.restart(ctx);
            println!("Player 2 wins!");
        }
        Ok(())
    }
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
    
    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
            )
    }
}

struct Stats {
    content_label: Text,
    position: Vec2<f32>,
    stats: Vec2<u32>,
}

impl Stats {
    fn new(stats: Option<Vec2<u32>>) -> Stats{
        match stats {
            Some(x) =>{
                Stats{
                    content_label: Text::new(x.to_string(), Font::default(), 15.0),
                    position: Vec2::new(10.0,10.0),
                    stats: x,
                }
            }
            None =>{
                Stats{
                    content_label: Text::new("test", Font::default(), 15.0),
                    position: Vec2::new(10.0,10.0),
                    stats: Vec2::new(1,1),
                }
            }
        }
    }

    fn update(&mut self, stats: Vec2<u32>) {
        self.stats = self.stats + stats;
        let score = self.stats.to_string();
        self.content_label.set_content(score);
    }
}
