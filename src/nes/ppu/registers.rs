use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct RegisterStatus {
    #[bits(5)]
    unused: usize,
    sprite_overflow: bool,
    sprite_zero_hit: bool,
    pub vertical_blank: bool
}

#[bitfield(u8)]
pub struct RegisterMask {
    pub grayscale: bool,
    render_background_left: bool,
    render_sprites_left: bool,
    pub render_background: bool,
    pub render_sprites: bool,
    enhance_red: bool,
    enhance_green: bool,
    enhance_blue: bool
}

#[bitfield(u8)]
pub struct RegisterControl {
    pub nametable_x: bool,
    pub nametable_y: bool,
    pub increment_mode: bool,
    pattern_sprite: bool,
    pub pattern_background: bool,
    sprite_size: bool,
    slave_mode: bool,
    pub enable_nmi: bool
}

#[bitfield(u16)]
pub struct RegisterLoopy {
    #[bits(5)]
    pub coarse_x: usize,
    #[bits(5)]
    pub coarse_y: usize,
    pub nametable_x: bool,
    pub nametable_y: bool,
    #[bits(3)]
    pub fine_y: usize,
    unused: bool
}
