use ggez::graphics::Color;

pub const BACKGROUND_COLOR: Color = Color {
    r: 41.0 / 255.0,
    g: 41.0 / 255.0,
    b: 41.0 / 255.0,
    a: 1.0,
};
pub const TILE_COLORS: (Color, Color) = (
    Color {
        r: 51.0 / 255.0,
        g: 51.0 / 255.0,
        b: 51.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 59.0 / 255.0,
        g: 59.0 / 255.0,
        b: 59.0 / 255.0,
        a: 1.0,
    },
);
pub const HEAD_COLOR: Color = TAIL_COLORS.1;
pub const TAIL_COLORS: (Color, Color) = (
    Color {
        r: 12.0 / 255.0,
        g: 185.0 / 255.0,
        b: 45.0 / 255.0,
        a: 1.0,
    },
    Color {
        r: 19.0 / 255.0,
        g: 138.0 / 255.0,
        b: 54.0 / 255.0,
        a: 1.0,
    },
);
pub const FRUIT_COLOR: Color = Color {
    r: 255.0 / 255.0,
    g: 87.0 / 255.0,
    b: 51.0 / 255.0,
    a: 1.0,
};

pub const TILE_SIZE: f32 = 50.0;
pub const BORDER_SIZE: f32 = 10.0;

pub const MILLISECONDS_PER_FRAME: usize = 300;
