use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("flappy dragon")
        .build()?;
    main_loop(context, State::new())
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
    score: i32,
    obstacle: Obstacle,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        // render background
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        // manage screen refresh rate
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        // fly flappy with space key
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        // render player
        self.player.render(ctx);
        // render info at top
        ctx.print(0, 0, "press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        // render obstacle
        self.obstacle.render(ctx, self.player.x);
        // check if player crossed the obstacle succesfully
        if self.player.x > self.obstacle.x {
            // increment the score
            self.score += 1;
            // add the next obstacle
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // check if player touched obstacle or dropped
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "wecome to Flappy Dragon");
        ctx.print_centered(9, "(P) Play");
        ctx.print_centered(13, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!!!");
        ctx.print_centered(5, &format!("you earned {}, points", self.score));
        ctx.print_centered(9, "(P) Play");
        ctx.print_centered(13, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    fn gravity_and_move(&mut self) {
        // if velocity is less, it fall increases
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        // flappy fall's relative to velocity
        self.y += self.velocity as i32;
        // horizontal distance increase because of inertia
        self.x += 1;
        // if flappy is going out of screen from above keep it inside
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        // for every flap it go's upwards 2 units
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let half_size = self.size / 2;
        let screen_x = self.x - player_x;

        // draw top half of Obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // draw bottom half of Obstacle
        for y in half_size + self.gap_y..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = self.x == player.x;
        let is_player_above_gap = player.y < self.gap_y - half_size;
        let is_player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (is_player_below_gap || is_player_above_gap)
    }
}
