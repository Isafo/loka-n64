use crate::bullet_system::BulletSystem;
use n64::{current_time_us, graphics, ipl3font, Controllers, Rng};
use n64_math::{Aabb2, Color, Vec2};
use crate::entity::{OwnedEntity, es};
use crate::components::movable;

const START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_COLOR: Color = Color::new(0b10000_00011_00011_1);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
pub const SHIP_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

pub struct Player {
    entity: OwnedEntity,
    last_shoot_time: i32,
    health: i32,
    score: i32,
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            entity: es().create_entity(),
            last_shoot_time: 0,
            health: 500,
            score: 0,
        };

        movable().add(&player.entity, START_POS, Vec2::zero());

        player
    }

    pub fn pos(&self) -> Vec2 {
        if let Some(movable) = movable().lookup_mut(&self.entity) {
            movable.pos
        } else {
            Vec2::zero()
        }
    }

    pub fn damage(&mut self, damage: i32) {
        self.health = 0.max(self.health - damage);
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn update(
        &mut self,
        dt: f32,
        controllers: &Controllers,
        bullet_system: &mut BulletSystem,
        rng: &mut Rng,
    ) {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if controller_x.abs() > 32 {
            controller_dir.set_x(if controller_x > 0 { 1.0 } else { -1.0 });
        }

        if controller_y.abs() > 32 {
            controller_dir.set_y(if controller_y > 0 { -1.0 } else { 1.0 });
        }

        if let Some(movable) = movable().lookup_mut(&self.entity) {

            movable.speed = SHIP_SPEED * controller_dir;

            {
                let now = current_time_us();

                if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS * 1000 {
                    if controllers.z() {
                        bullet_system.shoot_bullet(rng, movable.pos, Vec2::new(0.0, movable.speed.y() - 0.65));
                        self.last_shoot_time = now;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        if let Some(movable) = movable().lookup(&self.entity) {
            let screen_x = (movable.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
            let screen_y =
                (movable.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;

            ipl3font::draw_str(screen_x, screen_y, SHIP_COLOR, b"A");
        }
    }
}
