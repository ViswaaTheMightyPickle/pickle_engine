use std::collections::{BTreeMap, BTreeSet};

pub const CHUNK_SIZE: i32 = 32;
pub const CELLS_PER_METRE: f32 = 16.0;
pub const STEP_SECONDS: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialId(pub u16);

impl MaterialId {
    pub const EMPTY: Self = Self(0);
    pub const STONE: Self = Self(1);
    pub const SAND: Self = Self(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(pub u64);

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub x_cells: f32,
    pub y_cells: f32,
    pub angle_rad: f32,
}

// Arrays are row-major; y increases down the image/screen.
#[derive(Clone, Debug)]
pub struct ImportedObject {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<[u8; 4]>,
    pub materials: Vec<MaterialId>,
    pub collision: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct BodyState {
    pub id: ObjectId,
    pub transform: Transform,
}

#[derive(Clone, Copy, Debug)]
pub struct VisibleCell {
    pub pos: CellPos,
    pub material: MaterialId,
}

#[derive(Clone, Debug, Default)]
pub struct ViewSnapshot {
    pub cells: Vec<VisibleCell>,
    pub bodies: Vec<BodyState>,
}

#[derive(Clone, Debug)]
pub enum SceneCommand {
    Place {
        object: ImportedObject,
        at: Transform,
    },
    EraseObjectCell {
        id: ObjectId,
        local_x: u32,
        local_y: u32,
    },
    SetWorldCell {
        at: CellPos,
        material: MaterialId,
    },
    Reset,
}

pub fn chunk_of(pos: CellPos) -> ChunkPos {
    ChunkPos {
        x: pos.x.div_euclid(CHUNK_SIZE),
        y: pos.y.div_euclid(CHUNK_SIZE),
    }
}

fn local_index(pos: CellPos) -> usize {
    let x = pos.x.rem_euclid(CHUNK_SIZE) as usize;
    let y = pos.y.rem_euclid(CHUNK_SIZE) as usize;
    y * CHUNK_SIZE as usize + x
}

pub struct World {
    chunks: BTreeMap<ChunkPos, Vec<MaterialId>>,
    active_chunks: BTreeSet<ChunkPos>,
}

pub struct CellState {
    pub move_state: [bool; 3],
}

impl CellPos {
    pub fn to_physics_coordinates(self) -> PhysicsPos {
        let phy_x = self.x as f32 / CELLS_PER_METRE;
        let phy_y = self.y as f32 / CELLS_PER_METRE;

        PhysicsPos {
            x: (phy_x),
            y: (phy_y),
        }
    }

    pub fn to_physics_center(self) -> PhysicsPos {
        let phy_x = (self.x as f32 + 0.5) / CELLS_PER_METRE;
        let phy_y = (self.y as f32 + 0.5) / CELLS_PER_METRE;

        PhysicsPos {
            x: (phy_x),
            y: (phy_y),
        }
    }
}

impl PhysicsPos {
    pub fn to_cell_coordinates(self) -> CellPos {
        let cell_x = (self.x * CELLS_PER_METRE).floor() as i32;
        let cell_y = (self.y * CELLS_PER_METRE).floor() as i32;

        CellPos {
            x: (cell_x),
            y: (cell_y),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: BTreeMap::new(),
            active_chunks: BTreeSet::new(),
        }
    }

    pub fn get(&self, pos: CellPos) -> MaterialId {
        self.chunks
            .get(&chunk_of(pos))
            .map(|cells| cells[local_index(pos)])
            .unwrap_or(MaterialId::EMPTY)
    }

    pub fn set(&mut self, pos: CellPos, mat: MaterialId) {
        if self.get(pos) == mat {
            return;
        }

        if mat == MaterialId::EMPTY && !self.chunks.contains_key(&chunk_of(pos)) {
            return;
        }

        let chunk = self
            .chunks
            .entry(chunk_of(pos))
            .or_insert_with(|| vec![MaterialId::EMPTY; (CHUNK_SIZE * CHUNK_SIZE) as usize]);
        chunk[local_index(pos)] = mat;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let mut updated_position = pos;
                updated_position.x += dx;
                updated_position.y += dy;
                self.active_chunks.insert(chunk_of(updated_position));
            }
        }
    }

    pub fn is_in_world_boundaries(&self, pos: CellPos) -> bool {
        if pos.x > 255 || pos.x < 0 || pos.y > 255 || pos.y < 0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_solid(&self, pos: CellPos) -> bool {
        self.get(pos) == MaterialId::STONE
    }

    pub fn solid_cells_in_region(&self, min: CellPos, max: CellPos) -> Vec<CellPos> {
        let mut solid_cells: Vec<CellPos> = Vec::new();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.is_solid(CellPos { x, y }) {
                    solid_cells.push(CellPos { x, y });
                }
            }
        }

        solid_cells
    }

    pub fn check_cell_validity(&self, pos: CellPos) -> CellState {
        let below = CellPos {
            x: pos.x,
            y: pos.y + 1,
        };

        let diagonal_l = CellPos {
            x: pos.x - 1,
            y: pos.y + 1,
        };

        let diagonal_r = CellPos {
            x: pos.x + 1,
            y: pos.y + 1,
        };

        let can_move_left =
            self.is_in_world_boundaries(diagonal_l) && (self.get(diagonal_l) == MaterialId::EMPTY);
        let can_move_down =
            self.is_in_world_boundaries(below) && (self.get(below) == MaterialId::EMPTY);
        let can_move_right =
            self.is_in_world_boundaries(diagonal_r) && (self.get(diagonal_r) == MaterialId::EMPTY);

        CellState {
            move_state: [can_move_left, can_move_down, can_move_right],
        }
    }

    pub fn move_sand(&mut self, cell: VisibleCell, cell_state: CellState) -> Option<CellPos> {
        let position = cell.pos;

        let destination = if cell_state.move_state[1] {
            // Move down
            Some(CellPos {
                x: position.x,
                y: position.y + 1,
            })
        } else if cell_state.move_state[0] && cell_state.move_state[2] {
            let direction = if rand::random::<bool>() { -1 } else { 1 };
            Some(CellPos {
                x: position.x + direction,
                y: position.y + 1,
            })
        } else if cell_state.move_state[0] {
            // Move down-left
            Some(CellPos {
                x: position.x - 1,
                y: position.y + 1,
            })
        } else if cell_state.move_state[2] {
            // Move down-right
            Some(CellPos {
                x: position.x + 1,
                y: position.y + 1,
            })
        } else {
            None
        };

        if let Some(new_position) = destination {
            if self.get(position) == MaterialId::SAND && self.get(new_position) == MaterialId::EMPTY
            {
                self.set(position, MaterialId::EMPTY);
                self.set(new_position, MaterialId::SAND);
            }
        }
        None
    }

    pub fn step(&mut self) {
        let mut cells = self.sand_cells();
        let mut moved_positions = BTreeSet::<CellPos>::new();
        cells.sort_by(|a, b| b.pos.y.cmp(&a.pos.y));

        self.active_chunks.clear();

        for cell in cells {
            if cell.material == MaterialId::SAND && !moved_positions.contains(&cell.pos) {
                let state = self.check_cell_validity(cell.pos);
                if let Some(moved) = self.move_sand(cell, state) {
                    moved_positions.insert(moved);
                }
            }
        }

        self.chunks
            .retain(|_, chunk| chunk.iter().any(|mat| *mat != MaterialId::EMPTY));
    }

    pub fn visible_cells(&self) -> Vec<VisibleCell> {
        let mut result = Vec::new();
        for (&chunk_pos, cells) in &self.chunks {
            for (index, &material) in cells.iter().enumerate() {
                if material == MaterialId::EMPTY {
                    continue;
                }
                let x = (index % CHUNK_SIZE as usize) as i32;
                let y = (index / CHUNK_SIZE as usize) as i32;
                result.push(VisibleCell {
                    pos: CellPos {
                        x: chunk_pos.x * CHUNK_SIZE + x,
                        y: chunk_pos.y * CHUNK_SIZE + y,
                    },
                    material,
                });
            }
        }
        result
    }

    pub fn sand_cells(&self) -> Vec<VisibleCell> {
        let mut result = Vec::new();
        for &chunk_pos in &self.active_chunks {
            if let Some(cells) = self.chunks.get(&chunk_pos) {
                for (index, &material) in cells.iter().enumerate() {
                    if material != MaterialId::SAND {
                        continue;
                    }
                    let x = (index % CHUNK_SIZE as usize) as i32;
                    let y = (index / CHUNK_SIZE as usize) as i32;
                    result.push(VisibleCell {
                        pos: CellPos {
                            x: chunk_pos.x * CHUNK_SIZE + x,
                            y: chunk_pos.y * CHUNK_SIZE + y,
                        },
                        material,
                    });
                }
            }
        }
        result
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_coordinates_use_floor_chunks() {
        assert_eq!(
            chunk_of(CellPos { x: -1, y: -33 }),
            ChunkPos { x: -1, y: -2 }
        );
    }

    #[test]
    fn cells_round_trip_across_boundary() {
        let mut world = World::new();
        for x in [-33, -32, -1, 0, 31, 32, 33] {
            let pos = CellPos { x, y: 4 };
            world.set(pos, MaterialId::SAND);
            assert_eq!(world.get(pos), MaterialId::SAND);
        }
    }

    #[test]
    fn sand_falls_one_cell() {
        let mut world = World::new();

        world.set(CellPos { x: 5, y: 5 }, MaterialId::SAND);

        world.step();

        assert_eq!(world.get(CellPos { x: 5, y: 5 }), MaterialId::EMPTY);

        assert_eq!(world.get(CellPos { x: 5, y: 6 }), MaterialId::SAND);
    }

    #[test]
    fn sand_crosses_chunk_boundary() {
        let mut world = World::new();

        // Chunk (0,0) ends at y=31.
        world.set(CellPos { x: 10, y: 31 }, MaterialId::SAND);

        world.step();

        assert_eq!(world.get(CellPos { x: 10, y: 31 }), MaterialId::EMPTY);
        assert_eq!(world.get(CellPos { x: 10, y: 32 }), MaterialId::SAND);
    }

    #[test]
    fn sand_continues_after_crossing_chunk_boundary() {
        let mut world = World::new();

        world.set(CellPos { x: 10, y: 30 }, MaterialId::SAND);

        world.step();
        assert_eq!(world.get(CellPos { x: 10, y: 31 }), MaterialId::SAND);

        world.step();
        assert_eq!(world.get(CellPos { x: 10, y: 32 }), MaterialId::SAND);

        world.step();
        assert_eq!(world.get(CellPos { x: 10, y: 33 }), MaterialId::SAND);
    }

    #[test]
    fn blocked_sand_stays_stationary() {
        let mut world = World::new();

        let sand = CellPos { x: 10, y: 30 };

        world.set(sand, MaterialId::SAND);

        // Block all three possible destinations.
        for x in 9..=11 {
            world.set(CellPos { x, y: 31 }, MaterialId::STONE);
        }

        for _ in 0..10 {
            world.step();
        }

        assert_eq!(world.get(sand), MaterialId::SAND);
    }

    #[test]
    fn sleeping_sand_wakes_after_support_removed() {
        let mut world = World::new();

        let sand = CellPos { x: 10, y: 31 };

        world.set(sand, MaterialId::SAND);

        // The supporting cells are in another chunk.
        for x in 9..=11 {
            world.set(CellPos { x, y: 32 }, MaterialId::STONE);
        }

        // Sand should settle and its chunk should sleep.
        world.step();
        world.step();

        assert_eq!(world.get(sand), MaterialId::SAND);

        // Remove the supporting stone.
        world.set(CellPos { x: 10, y: 32 }, MaterialId::EMPTY);

        // The sand must wake and fall into the gap.
        world.step();

        assert_eq!(world.get(sand), MaterialId::EMPTY);
        assert_eq!(world.get(CellPos { x: 10, y: 32 }), MaterialId::SAND);
    }

    #[test]
    fn sand_moves_only_once_per_step() {
        let mut world = World::new();

        world.set(CellPos { x: 15, y: 15 }, MaterialId::SAND);

        world.step();

        assert_eq!(world.get(CellPos { x: 15, y: 16 }), MaterialId::SAND);

        assert_eq!(world.get(CellPos { x: 15, y: 17 }), MaterialId::EMPTY);
    }

    #[test]
    fn sand_count_is_conserved() {
        let mut world = World::new();

        // A small block of sand.
        for x in 10..15 {
            for y in 10..15 {
                world.set(CellPos { x, y }, MaterialId::SAND);
            }
        }

        let initial_count = world.sand_cells().len();

        for _ in 0..100 {
            world.step();
        }

        let final_count = world
            .visible_cells()
            .iter()
            .filter(|cell| cell.material == MaterialId::SAND)
            .count();

        assert_eq!(initial_count, final_count);
    }
}
