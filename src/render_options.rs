#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionKind {
    Orthographic,
    Perspective,
}

impl Default for ProjectionKind {
    fn default() -> Self {
        Self::Orthographic
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub projection_kind: ProjectionKind,
    pub perspective_fov: f32,
    pub background_color: Color,
    pub edge_color: Color,
    pub vertex_color: Color,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            projection_kind: ProjectionKind::default(),
            perspective_fov: 90.0,
            background_color: Color::new(0x3a, 0x3a, 0x3a),
            edge_color: Color::new(0xff, 0xff, 0xff),
            vertex_color: Color::new(0x49, 0x95, 0xdd),
        }
    }
}

impl RenderOptions {
    pub fn set_projection_kind(&mut self, kind: ProjectionKind) {
        self.projection_kind = kind;
    }

    pub fn set_perspective_fov(&mut self, fov: f32) {
        self.perspective_fov = fov.clamp(45.0, 110.0);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_edge_color(&mut self, color: Color) {
        self.edge_color = color;
    }

    pub fn set_vertex_color(&mut self, color: Color) {
        self.vertex_color = color;
    }
}
