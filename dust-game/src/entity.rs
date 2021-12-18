pub trait Entity {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

pub trait LivingEntity: Entity {
    fn get_health(&self) -> u8;
}

pub trait AiEntity: LivingEntity {
    // noop
}

pub trait BotEntity: AiEntity {
    // noop
}

pub trait ProjectileEntity: AiEntity {
    // noop
}

pub trait PlayerEntity: LivingEntity {
    fn get_name(&self) -> str;
}
