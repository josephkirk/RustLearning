// source: https://www.youtube.com/watch?v=gKNJKce1p8M

use bevy::prelude::*;
use bevy::prelude::Srgba;
use rand::prelude::*;
use std::collections::HashMap;
use strum::FromRepr;
const CELLSIZE:i32 = 5;
const ITERATION:i32 = 8;
const NCELLSEARCHRANGE: usize = 3;
const NCELLNEIGHTBOR: usize = NCELLSEARCHRANGE*NCELLSEARCHRANGE;
#[derive(Component)]
struct Cell {
    x: i32,
    y: i32,
    value: MapCellType,
    neightbors: Vec<Entity>
}

fn hash_coord(x:i32,y:i32) -> String {
    format!("{x}_{y}")
}

#[derive(Default,Debug, Clone, Copy, Eq, PartialEq, Hash, FromRepr)]
enum MapCellType {
    #[default]
    Undeclared,
    Moutains,
    Forest,
    Plains,
    Water,
    DeepWater,
    HighMountains
}

const MAPCELLTABLE: [[usize;7];7] = [
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1],
    [0,0,0,0,0,0,1]
];

fn map_table(cell_type: MapCellType, other_cell_type: MapCellType) -> usize {
    MAPCELLTABLE[cell_type.index()][other_cell_type.index()]
}

fn get_cell_num_type() -> i32 {
    7
}

impl MapCellType {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn check_conflict(&self, other_cell_type: MapCellType) -> usize {
        map_table(*self, other_cell_type)
    }

    pub fn color(&self) -> Color {
        let cell_color = match self {
            MapCellType::Undeclared => "000000",
            MapCellType::Moutains => "AEC2B6",
            MapCellType::Forest => "609966",
            MapCellType::HighMountains => "BDCDD6",
            MapCellType::Plains => "BBD6B8",
            MapCellType::DeepWater => "93BFCF",
            MapCellType::Water => "6096B4",
        };
        Color::from(Srgba::hex(cell_color).unwrap_or_default())
    }
}

#[derive(Resource, Default, Clone)]
struct Map {
    width: i32,
    height: i32,
    cells: HashMap<String, Entity>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            find_neighbors_cell,
            find_least_cellconflict,
            draw_map
        ).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {
    let (width, height) = (800, 1000);
    let world_width = width / CELLSIZE;
    let world_height = height / CELLSIZE;
    let map = Map {
        width: world_width,
        height: world_height,
        cells: {
            let mut _cells:HashMap<String, Entity> = HashMap::new();
            // Map need a way to query current cell value from x,y
            for x in (1..world_width) {
                for y in (1..world_height) {
                    let cell = Cell {
                        x:x,
                        y:y,
                        value:MapCellType::Undeclared,
                        neightbors:Vec::new()
                    };
                    let cell_entity = commands.spawn((cell));
                    let cell_hash = hash_coord(x, y);
                    _cells.insert(cell_hash, cell_entity.id());
                }
            };
            _cells
        }
    };
    commands.insert_resource(map);
}

fn draw_map()
{

}

fn find_neighbors_cell(
    mut query: Query<(&mut Cell)>,
    map: Res<Map>,
) {
    for (mut cell) in query.iter_mut() {
        let x = cell.x;
        let y = cell.y;
        let search_range = 3;
        for dx in (-search_range..search_range) {
            for dy in (-search_range..search_range) {
                let tx = (dx + x + map.width) % map.width;
                let ty = (dy + y + map.height) % map.height;
                let cell_hash = hash_coord(tx, ty);
                let cell_entity = map.cells[&cell_hash];
                cell.neightbors.push(cell_entity);
            }
        }
    }
}

fn find_least_cell_conflict(
    mut commands: Commands,
    mut query: Query<(&mut Cell)>,
    mut map: ResMut<Map>,
) -> bool {
    let success = false;

    for i in (0..query.iter().count())
    {
        let x = rand::thread_rng().gen_range(0..map.width);
        let y = rand::thread_rng().gen_range(0..map.height);
        let cell_hash = hash_coord(x, y);
        let cell_entity = map.cells[&cell_hash];
        let conflicts = check_conflict_cells(cell_entity, &query, &map);
        if conflicts > 0 {
            let mut best_type = MapCellType::Undeclared;
            let mut least_conflicts = 100;
            let (mut temp_terrain, mut temp_conflicts) = (MapCellType::Undeclared,0);
            for j in (0..ITERATION) {
                let temp_terrain_num = 1 + rand::thread_rng().gen_range(0..(get_cell_num_type()-1));
                temp_terrain = MapCellType::from_repr(temp_terrain_num as usize).unwrap_or_default();
                temp_conflicts = check_conflicts(x, y, map.width, map.height);
                if (temp_conflicts < conflicts) {
                    best_type = temp_terrain;
                    least_conflicts = temp_conflicts;
                }
            };
            if let Ok(mut selected_cell) = query.get_mut(cell_entity) {
                selected_cell.value = best_type;
                
            }
        }
    }
    success
}

fn check_conflict_cells(
    check_cell: Entity,
    query: &Query<(&mut Cell)>,
    mut map: &ResMut<Map>) -> i32
{
    let mut conflicts = 0;
    if let Ok(mut selected_cell) = query.get_mut(check_cell) {
        let neightbors_cell_array: Result<[Entity; NCELLNEIGHTBOR], _> = selected_cell.neightbors.try_into();
        let neightbor_cell_entities:  = neightbors_cell_array.unwrap_err();

        if let Ok(mut neightbor_cells) = query.get_many_mut(neightbor_cell_entities)
        {
            for neightbor_cell in neightbor_cells.iter() {

            }
        }
    }
    conflicts
}

fn check_conflicts(x:i32, y:i32, width:i32, height:i32) -> i32{
    let mut conflicts = 0;
    let range = 3;
    for dx in (-range..range) {
        for dy in (-range..range) {
            let tx = (dx + x + width) % width;
            let ty = (dy + y + height) % height;
            conflicts += 1; //Replace with MapCelType::check_conflicts
        }
    }
    conflicts
} 