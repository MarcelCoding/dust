// pub use self::{};
pub use crate::entity::AiEntity;
pub use crate::entity::Entity;
pub use crate::entity::LivingEntity;
pub use crate::position::Position;

pub struct BotEntity {
    pos: Position<f32>,
    health: u8,
}

impl Entity for BotEntity {
    fn get_pos(&self) -> &Position<f32> {
        &self.pos
    }
}

impl LivingEntity for BotEntity {
    fn get_health(&self) -> u8 {
        self.health
    }
}

impl AiEntity for BotEntity {}
