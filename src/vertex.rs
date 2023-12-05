/// A vertex sent to the vertex shader through the vertex buffer, and its fields layout.
#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    /// the id of the entity this vertex belongs to.
    pub entity_id: u32,
    /// The initial position of the vertex on the XY plane
    pub position: [f32 ; 2],
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32) -> Self {
        Self {
            entity_id: 0,
            position: [x, y],
        }
    }
}

impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nVertex: \nentity_id = {}\nposition = ({}, {})\n", self.entity_id, self.position[0], self.position[1])
    }
}