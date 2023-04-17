use crate::voxel::Chunk;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ChunkManager {
    map: HashMap<IVec3, Chunk>,
}
