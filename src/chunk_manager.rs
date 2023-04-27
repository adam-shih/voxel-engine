use crate::voxel::Chunk;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use std::collections::HashMap;

const RENDER_DISTANCE: u32 = 5;

pub struct ChunkManager {
    loaded: HashMap<IVec3, Chunk>,
}

impl ChunkManager {}
