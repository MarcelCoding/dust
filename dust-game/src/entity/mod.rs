pub use bot::BotEntity;

pub use crate::position::Position;

mod bot;
mod player;
mod projectile;

pub trait Entity {
    fn get_pos(&self) -> &Position<f32>;
}

pub trait LivingEntity: Entity {
    fn get_health(&self) -> u8;
}

pub trait AiEntity: LivingEntity {}
