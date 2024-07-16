// source: https://www.youtube.com/watch?v=gKNJKce1p8M

use bevy::prelude::*;
use bevy::prelude::Srgba;
use rand::prelude::*;
use std::collections::HashMap;

const CELLSIZE:i32 = 5;
const ITERATION:i32 = 8;

#[derive(Component)]
struct Cell {
    x: i32,
    y: i32,
    value: MapCellType
}

fn hash_coord(x:i32,y:i32) -> String {
    format!("{x}_{y}")
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum MapCellType {
    Undeclared,
    Moutains,
    Forest,
    Plains,
    Water,
    DeepWater,
    HighMountains
}

fn get_cell_num_type() -> i32 {
    7
}

fn find_cell_shade(cell_type:MapCellType) -> Color {
    let cell_color = match cell_type {
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

#[derive(Resource, Default)]
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
            draw_map
        ))
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
                        value:MapCellType::Undeclared
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

fn find_least_conflict(
    mut commands: Commands,
    mut query: Query<&mut Cell>,
    mut map: ResMut<Map>,
) -> bool {
    let success = false;

    for mut cell in &mut query
    {
        let x = rand::thread_rng().gen_range(0..map.width);
        let y = rand::thread_rng().gen_range(0..map.height);
        let cell_hash = hash_coord(x, y);
        let cell_entity = map.cells[&cell_hash];

        let conflicts = check_conflicts(x, y);
        if conflicts > 0 {
            let mut best_type: MapCellType;
            let mut least_conflicts: i32 = 100;
            let (mut temp_terrain, mut temp_conflicts) = (MapCellType::Undeclared,0);
            for j in (0..ITERATION) {
                temp_terrain = 1 + rand::thread_rng().gen_range(0..(get_cell_num_type()-1));
                temp_conflicts = check_conflicts(x, y);
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

fn check_conflicts(x:i32, y:i32) -> i32{
    0
} 