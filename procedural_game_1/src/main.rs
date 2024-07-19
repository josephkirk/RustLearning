// Map generation sample using Constraint Satisfaction Algorithm
// follow instruction from source: https://www.youtube.com/watch?v=gKNJKce1p8M

use bevy::{
    log::LogPlugin, prelude::{Srgba, *}, sprite::MaterialMesh2dBundle
};
use rand::prelude::*;
use std::collections::HashMap;
use strum::FromRepr;

const CELLSIZE:i32 = 5;
const ITERATION:i32 = 8;
const NCELLSEARCHRANGE: usize = 3;
const MAPCELLTYPES:usize = 8;
const MAPWIDTH:i32 = 800;
const MAPHEIGHT:i32 = 1000;


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
    Sand,
    Water,
    DeepWater,
    HighMountains
}

const MAPCELLCONFLICTTABLE: [[usize;MAPCELLTYPES];MAPCELLTYPES] = [
    [0,0,0,0,0,0,0,0],
    [0,0,0,1,1,1,1,0],
    [0,0,0,0,1,1,1,1],
    [0,1,0,0,0,1,1,1],
    [0,1,1,0,0,0,1,1],
    [0,1,1,1,0,0,0,1],
    [0,1,1,1,1,0,0,1],
    [0,0,1,1,1,1,1,0],
];

fn map_table(cell_type: MapCellType, other_cell_type: MapCellType) -> usize {
    MAPCELLCONFLICTTABLE[cell_type.index()][other_cell_type.index()]
}

fn get_cell_num_type() -> i32 {
    MAPCELLTYPES as i32
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
            MapCellType::HighMountains => "FFFFFF",
            MapCellType::Plains => "BBD6B8",
            MapCellType::Sand => "E7D4B5",
            MapCellType::DeepWater => "134B70",
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
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                               primary_window:Some(
                                    Window{
                                        title:"Procedural Test".into(),
                                        name:Some("proceduralapp".into()),
                                        resolution:(MAPWIDTH as f32,MAPHEIGHT as f32).into(),
                                        ..default()
                                    }),
                                ..default()})
                           .set(LogPlugin{
                               filter:"info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                               level: bevy::log::Level::DEBUG,..default()
                            }))
        .add_systems(Startup, (
            setup,
            setup_map
        ).chain())
        .add_systems(Update, (
            update_map.run_if(is_map_generated),
            re_update_map
        ).chain())
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .run();
}

fn setup(
    world: &mut World
) {
    let (width, height) = (MAPWIDTH, MAPHEIGHT);
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(400.,500.,0.),
        ..default()
    });
    for (coord_hash, cell) in map.cells.iter() {
        let color = cell.value.color();
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::splat(CELLSIZE as f32))).into(),
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                (cell.x*CELLSIZE+ 10 as i32) as f32,
                (cell.y*CELLSIZE+ 10 as i32) as f32,
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
        let cell_hash = &cell_component.0;
        let cell = map.cells[cell_hash];
        let cell_type = cell.value;
        let cell_color = cell_type.color();
        if let Some(material) = materials.get_mut(material_handle)
        {
            material.color = cell_color;
            debug!("MAPGEN:: update {cell_hash} color to {:?}\n", cell_color);
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
        for dy in -search_range..search_range {
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

// TODO: refactor this function into background task call to prevent
#[allow(unused_assignments)]
fn find_least_cell_conflict(
    map: &mut Map,
){
    let mut conflict_count = 0;
    for _ in 0..map.cells.len()
    {
        let x = rand::thread_rng().gen_range(0..map.width);
        let y = rand::thread_rng().gen_range(0..map.height);
        let cell_hash = hash_coord(x, y);
        let tries = ITERATION;
        debug!("MAPGEN:: find least conflict for {cell_hash} ");
        
        if map.cells.contains_key(&cell_hash)
        {
            let mut selected_cell = map.cells[&cell_hash];
            
            let conflicts: i32 = check_conflicts(&selected_cell, &map) as i32;
        
            if conflicts > 0 || selected_cell.value == MapCellType::Undeclared
            {
                conflict_count += 1;
                let mut best_type = MapCellType::Undeclared;
                let mut least_conflicts: i32 = 100;
                let mut temp_terrain = MapCellType::Undeclared;
                let mut temp_conflicts: i32 = 0;
                for _ in 0..tries {
                    let temp_terrain_num = rand::thread_rng().gen_range(1..get_cell_num_type());
                    temp_terrain = MapCellType::from_repr(temp_terrain_num as usize).unwrap_or_default();
                    selected_cell.value = temp_terrain;
                    temp_conflicts = check_conflicts(&selected_cell, &map) as i32;
                    if temp_conflicts < least_conflicts 
                    {
                        best_type = temp_terrain;
                        least_conflicts = temp_conflicts;
                    }
                };
                if let Some(target_cell) = map.cells.get_mut(&cell_hash)
                {
                    target_cell.value = best_type
                }
                debug!(":: found best type {:?}\n", selected_cell.value);
            } 
        }
    }
    if conflict_count > 0
    {
        map.iteration += 1;
    } else {
        map.is_generated = true;
        info!("MAPGEN:: Map generation finished after {} iteration!\n", map.iteration);
    }
    debug!("MAPGEN:: Current Iteration {} current conflict count {}", map.iteration, conflict_count)
}

fn re_update_map(
    buttons: Res<ButtonInput<MouseButton>>,
    mut map: ResMut<Map>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if map.is_generated
        {
            map.is_generated = false;
            map.iteration = 0;
            map.cells = {
                let mut _cells:HashMap<String, Cell> = HashMap::new();
                // Map need a way to query current cell value from x,y
                for x in 0..map.width {
                    for y in 0..map.height {
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
            };
            info!("MAPGEN:: Regenerating Map ...")
        }
    }
}