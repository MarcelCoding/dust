// pub use self::{};
pub use crate::entity::Entity;
pub use crate::entity::LivingEntity;
pub use crate::position::Position;

pub struct PlayerEntity {
    pos: Position<f32>,
    health: u8,
}

impl Entity for PlayerEntity {
    fn get_pos(&self) -> &Position<f32> {
        &self.pos
    }
}

impl LivingEntity for PlayerEntity {
    fn get_health(&self) -> u8 {
        self.health
    }
}
