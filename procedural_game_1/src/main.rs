// source: https://www.youtube.com/watch?v=gKNJKce1p8M

use bevy::{
    prelude::{Srgba, *},
    sprite::MaterialMesh2dBundle
};
use rand::prelude::*;
use std::collections::HashMap;
use strum::FromRepr;

const CELLSIZE:i32 = 5;
const ITERATION:i32 = 8;
const NCELLSEARCHRANGE: usize = 3;
const MAXMAPGENITERATION: i32 = 10;

#[derive(Debug, Default, Clone, PartialEq, Eq, Copy)]
struct Cell {
    x: i32,
    y: i32,
    value: MapCellType,
}

#[derive(Component)]
struct CellComponent (String);

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

const MAPCELLCONFLICTTABLE: [[usize;7];7] = [
    [0,0,0,0,0,0,0],
    [0,0,0,1,1,1,0],
    [0,0,0,0,1,1,1],
    [0,1,0,0,0,1,1],
    [0,1,1,0,0,0,1],
    [0,1,1,1,0,0,1],
    [0,0,1,1,1,1,0]
];

fn map_table(cell_type: MapCellType, other_cell_type: MapCellType) -> usize {
    MAPCELLCONFLICTTABLE[cell_type.index()][other_cell_type.index()]
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
    cells: HashMap<String, Cell>,
    is_generated: bool,
    iteration: i32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{primary_window:Some(Window{title:"Procedural Test".into(), name:Some("proceduralapp".into()),resolution:(800.,1000.).into(),..default()}),..default()}))
        .add_systems(Startup, (
            setup,
            setup_map
        ).chain())
        .add_systems(FixedUpdate, (
            update_map.run_if(is_map_generated),
        ).chain())
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .run();
}

fn setup(
    world: &mut World
) {
    let (width, height) = (800, 1000);
    let world_width = width / CELLSIZE;
    let world_height = height / CELLSIZE;
    let map = Map {
        width: world_width,
        height: world_height,
        cells: {
            let mut _cells:HashMap<String, Cell> = HashMap::new();
            // Map need a way to query current cell value from x,y
            for x in 0..world_width {
                for y in 0..world_height {
                    let cell = Cell {
                        x:x,
                        y:y,
                        value:MapCellType::Undeclared,
                    };
                    let cell_hash = hash_coord(x, y);
                    _cells.insert(cell_hash, cell);
                }
            };
            _cells
        },
        ..default()
    };
    world.insert_resource(map);
    
}

fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
)
{
    commands.spawn(Camera2dBundle::default());
    for (coord_hash, cell) in map.cells.iter() {
        let color = cell.value.color();
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::splat(CELLSIZE as f32))).into(),
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                (cell.x+CELLSIZE as i32) as f32,
                (cell.y+CELLSIZE as i32) as f32,
                0.0,
            ),
            ..default()
        }, CellComponent(coord_hash.into())));
    }
}

fn update_map(
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&CellComponent, &Handle<ColorMaterial>)>,
    mut map: ResMut<Map>,
) {
    for(cell_component , material_handle) in query.iter() {
        let cell = map.cells[&cell_component.0];
        let cell_type = cell.value;
        if let Some(material) = materials.get_mut(material_handle)
        {
            material.color = cell_type.color();
        }
    }
    find_least_cell_conflict(&mut map);
}

fn is_map_generated(
    map: Res<Map>
)-> bool {
    !map.is_generated
}

fn check_conflicts(
    cell: &Cell,
    map: &Map
) -> usize {
    let mut conflicts = 0;
    let search_range = NCELLSEARCHRANGE as i32;
    for dx in -search_range..search_range {
        for dy in (-search_range..search_range) {
            let tx = (dx + cell.x + map.width) % map.width;
            let ty = (dy + cell.y + map.height) % map.height;
            let cell_hash = hash_coord(tx, ty);
            if map.cells.contains_key(&cell_hash)
            {
                let check_cell = map.cells[&cell_hash];
                conflicts += cell.value.check_conflict(check_cell.value);
            }
        }
    }
    conflicts
}

fn find_least_cell_conflict(
    map: &mut Map,
){
    if map.iteration >= MAXMAPGENITERATION {
        print!("Map generation finished !\n");
        map.is_generated = true;
        return
    }
    for _ in 0..map.cells.len()
    {
        let x = rand::thread_rng().gen_range(0..map.width);
        let y = rand::thread_rng().gen_range(0..map.height);
        let cell_hash = hash_coord(x, y);
        print!("find least conflict for {cell_hash} ");
        if map.cells.contains_key(&cell_hash)
        {
            let mut selected_cell = map.cells[&cell_hash];
            
            let conflicts = check_conflicts(&selected_cell, &map);
            if conflicts > 0 || selected_cell.value == MapCellType::Undeclared
            {
                let mut best_type = MapCellType::Undeclared;
                let mut least_conflicts = 100;
                let mut temp_terrain = MapCellType::Undeclared;
                let mut temp_conflicts = 0;
                for _ in 0..ITERATION {
                    let temp_terrain_num = 1 + rand::thread_rng().gen_range(0..(get_cell_num_type()-1));
                    temp_terrain = MapCellType::from_repr(temp_terrain_num as usize).unwrap_or_default();
                    temp_conflicts = check_conflicts(&selected_cell, &map);
                    if temp_conflicts < least_conflicts 
                    {
                        best_type = temp_terrain;
                        least_conflicts = temp_conflicts;
                    }
                };
                selected_cell.value = best_type;
                print!(":: found best type {:?}\n", best_type);
            }
        }
    }
    map.iteration += 1;
}
