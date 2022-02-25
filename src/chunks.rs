
use std::collections::HashMap;
use bevy::prelude::*;
use crate::resources::*;
use crate::tile_factory::*;
use crate::game::GameData;
use bevy::{
    asset::{AssetLoader,LoadContext,LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture
};

#[derive(serde::Deserialize,Clone)]
pub enum TileKind {
    Tree,
    Bush,
    Sign(String),
}

impl TileKind {
    pub fn from_default_char(c:char) -> Option<Self> {
        match c {
            '#' => {
                Some(TileKind::Bush)
            }
            'T' => {
                Some(TileKind::Tree)
            }
            _ => {None}
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ChunkData {
    pub custom_tiles: HashMap<char,TileKind>,
    pub tiles: String,
}

impl ChunkData {
    pub fn as_kinds(&self) -> Vec<((u8,u8),TileKind)> {
        let mut kinds:Vec<((u8,u8),TileKind)> = Vec::new();
            let mut y:u8 = 0;
            for line in self.tiles.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    let mut x:u8 = 0;
                    for c in line.chars() {
                        let mut tile: Option<TileKind> = self.custom_tiles.get(&c).cloned();
                        if tile.is_none() {
                            tile = TileKind::from_default_char(c);
                        }
                        if let Some(tile) = tile {
                            kinds.push(((x,y),tile));
                        }
                        x+=1;
                        if x > 13 {break;}
                    }
                    y+=1;
                }
                if y > 8 {break;}
            }
        kinds
    }
}

#[derive(serde::Deserialize)]
#[derive(TypeUuid)]
#[uuid = "967e1ae8-95aa-11ec-b909-0242ac120002"]
pub struct ChunkMap{
    pub chunks: std::collections::HashMap<(i32,i32),ChunkData>,
    pub default_chunk: ChunkData,
}

pub struct ChunkManager {
    pub loaded_chunks:    Vec<((i32,i32),Entity)>,
    pub active_chunk:     (i32,i32),
    pub player_chunk:     (i32,i32),
}

impl ChunkManager {
    pub fn is_loaded(&self,chunk:&(i32,i32)) -> bool {
        for (loaded_chunk,_) in self.loaded_chunks.iter() {
            if loaded_chunk == chunk {
                return true;
            }
        }
        false
    }
    pub fn shift_player(&mut self,dx:i32,dy:i32){
        self.player_chunk = (
            self.player_chunk.0 + dx,
            self.player_chunk.1 + dy
        );
        //dbg!(&self.player_chunk);
    }
}

fn tile_transform(col:u8,row:u8) -> Transform {
    Transform::from_translation(
        Vec3::new(col as f32 * 16.-104.,64.-row as f32 * 16.,0.)
    )
}

pub fn load_chunk(
    cmd:       &mut Commands,
    atlas_map: &AtlasMap,
    chunk:     (i32,i32),
    chunkmap: &ChunkMap,
) -> Entity{
    cmd.spawn().insert(GlobalTransform::default())
    .insert(Transform::from_translation(Vec3::new(
        chunk.0 as f32 * 224., chunk.1 as f32 * 144., chunk.1 as f32 *-0.5,
    ))).with_children(|root|{
        let mut spawn_chunk = |chunk_data:&ChunkData| {
            for ((x,y),tile_kind) in chunk_data.as_kinds() {
                match tile_kind {
                    TileKind::Tree => {
                        root.spawn_bundle(TreeBundle::new(&atlas_map,
                            tile_transform(x,y)));
                    }
                    TileKind::Bush => {
                        root.spawn_bundle(BushBundle::new(&atlas_map,
                            tile_transform(x,y)));
                    }
                    TileKind::Sign(text) => {
                        root.spawn_bundle(SignBundle::new(
                            &atlas_map,
                            tile_transform(x,y),
                            text
                        ));
                    }
                }
            }
        };
        if let Some(chunk_data) = chunkmap.chunks.get(&chunk) {
            spawn_chunk(&chunk_data);
        } else {
            spawn_chunk(&chunkmap.default_chunk);
        }
    }).id()
}

pub fn chunk_manager(
    mut cmd: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    atlas_map: Res<AtlasMap>,
    chunk_assets: Res<Assets<ChunkMap>>,
    game_data: Res<GameData>,
) {
    if let Some(chunkmap) = chunk_assets.get(&game_data.chunkmap_handle) {
        if chunk_manager.player_chunk != chunk_manager.active_chunk {
            //freeze active_chunk

            //set new chunk to active
            chunk_manager.active_chunk = chunk_manager.player_chunk;

            //load neighboring chunks and confirm active chunk is loaded
            let (x,y) = chunk_manager.active_chunk;
            let buffer = 2;
            let mut neighbors = Vec::new();
            for i in (x-buffer)..=(x+buffer) {
                for j in (y-buffer)..=(y+buffer) {
                    let chunk = (i,j);
                    if !chunk_manager.is_loaded(&chunk) {
                        let chunk_root = load_chunk(&mut cmd,&atlas_map,chunk,chunkmap);
                        chunk_manager.loaded_chunks.push(((i,j),chunk_root));
                    }
                    neighbors.push(chunk);
                }
            }
            
            chunk_manager.loaded_chunks = chunk_manager.loaded_chunks.iter().filter(|(pos,root_entity)|{
                if neighbors.contains(&pos) {
                    return true;
                } else {
                    //println!("despawning chunk: {:?}",pos);
                    cmd.entity(*root_entity).despawn_recursive();
                    return false;
                }
            }).map(|r|*r).collect();

            //unfreeze active_chunk
        }
    }
}
