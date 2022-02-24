
use bevy::prelude::*;
use crate::resources::*;
use crate::tile_factory::*;
use crate::game::GameData;
use bevy::{
    asset::{AssetLoader,LoadContext,LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture
};

#[derive(serde::Deserialize)]
#[derive(TypeUuid)]
#[uuid = "967e1ae8-95aa-11ec-b909-0242ac120002"]
pub struct ChunkMap{
    pub chunks: std::collections::HashMap<(i32,i32),String>,
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
        let mut string_to_chunk = |string:&String| {
            let mut y:u8 = 0;
            for line in string.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    let mut x:u8 = 0;
                    for c in line.chars() {
                        match c {
                            '#' => {
                                root.spawn_bundle(BushBundle::new(&atlas_map,
                                        tile_transform(x,y)));
                            }
                            'T' => {
                                root.spawn_bundle(TreeBundle::new(&atlas_map,
                                        tile_transform(x,y)));
                            }
                            '.' => {

                            }
                            _ => {dbg!(&c);}
                        }
                        x+=1;
                        if x > 13 {break;}
                    }
                    y+=1;
                }
                if y > 8 {break;}
            }
        };
        if let Some(chunk_string) = chunkmap.chunks.get(&chunk) {
            string_to_chunk(chunk_string);
        }
        /*
        match chunk {
            (0,0) => {
                string_to_chunk(r#"
                    ##T##T.T.T.T.T
                    T###T.T.T.T.T.
                    ###T#.........
                    ##T##.........
                    ###T#.........
                    ##T#..........
                    ###T#.........
                    #T###.T.T.T.T.
                    ###T#T.T.T.T.T
                "#.to_string())
            }

            (_,-3) => {
                string_to_chunk(r#"
                    .#.#.###......
                    .###..#.......
                    .#.#.###......
                    ..............
                    #.#..#..###.##
                    #.#.#.#..#..#.
                    ##..###..#..##
                    #.#.#.#..#..#.
                    #.#.#.#..#..##
                "#.to_string())
            }
            _ => {}
        }
        */
    }).id()
}

pub fn chunk_map_debug(
    chunk_assets: Res<Assets<ChunkMap>>,
    game_data: Res<GameData>,
) {
    let chunkmap = chunk_assets.get(&game_data.chunkmap_handle);
    if let Some(chunkmap) = chunkmap {
        dbg!(&chunkmap.chunks);
    }
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
