use std::collections::BTreeMap;

pub const CHUNK_SIZE: i32 = 32;
pub const CELLS_PER_METRE: f32 = 16.0;
pub const STEP_SECONDS: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
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
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: BTreeMap::new(),
        }
    }

    pub fn get(&self, pos: CellPos) -> MaterialId {
        self.chunks
            .get(&chunk_of(pos))
            .map(|cells| cells[local_index(pos)])
            .unwrap_or(MaterialId::EMPTY)
    }

    pub fn set(&mut self, pos: CellPos, material: MaterialId) {
        if material == MaterialId::EMPTY && !self.chunks.contains_key(&chunk_of(pos)) {
            return;
        }

        let chunk = self
            .chunks
            .entry(chunk_of(pos))
            .or_insert_with(|| vec![MaterialId::EMPTY; (CHUNK_SIZE * CHUNK_SIZE) as usize]);
        chunk[local_index(pos)] = material;
    }

    pub fn step(&mut self) {
        // TODO (Viswaa): sand movement, including chunk crossings.
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
}
