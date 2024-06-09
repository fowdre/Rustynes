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
    grayscale: bool,
    render_background_left: bool,
    render_sprites_left: bool,
    render_background: bool,
    render_sprites: bool,
    enhance_red: bool,
    enhance_green: bool,
    enhance_blue: bool
}

#[bitfield(u8)]
pub struct RegisterControl {
    nametable_x: bool,
    nametable_y: bool,
    pub increment_mode: bool,
    pattern_sprite: bool,
    pattern_background: bool,
    sprite_size: bool,
    slave_mode: bool,
    pub enable_nmi: bool
}
