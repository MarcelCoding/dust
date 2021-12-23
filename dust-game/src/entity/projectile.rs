pub use crate::position::Position;
pub use crate::entity::Entity;
pub use crate::entity::LivingEntity;
pub use crate::entity::AiEntity;

pub struct ProjectileEntity {
    pos: Position<f32>
}

impl Entity for ProjectileEntity {
    fn get_pos(&self) -> &Position<f32> {
        &self.pos
    }
}

impl LivingEntity for ProjectileEntity {
    fn get_health(&self) -> u8 {
        1
    }
}

impl AiEntity for ProjectileEntity {}
