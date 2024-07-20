// Map generation sample using Constraint Satisfaction Algorithm
// follow instruction from source: https://www.youtube.com/watch?v=gKNJKce1p8M

use bevy::{
    ecs::{entity, system::{RunSystemOnce, SystemId}, world::CommandQueue}, log::LogPlugin, math::I64Vec2, prelude::{Srgba, *}, sprite::MaterialMesh2dBundle, tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, ComputeTaskPool, Task}, transform::commands
};
use rand::prelude::*;
use std::collections::HashMap;
use strum::FromRepr;

const CELLSIZE:usize = 5;
const ITERATION:i32 = 8;
const NCELLSEARCHRANGE: usize = 3;
const MAPCELLTYPES:usize = 8;
const MAPWIDTH:usize = 800;
const MAPHEIGHT:usize = 1000;

#[derive(Event,Default)]
struct MapGenTaskFinished();

#[derive(Event,Default)]
struct ShouldGenMapEvent();

#[derive(Debug, Default, Clone, PartialEq, Eq, Copy)]
struct Cell {
    x: i32,
    y: i32,
    value: MapCellType,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct MapChunkResult {
    map: HashMap<I64Vec2, MapCellType>,
    conflicts_count: i64
}

impl Cell {
    fn hash(&self) -> String {
        hash_coord(self.x, self.y)
    }
}

#[derive(Component)]
struct CellComponent {
    coord: I64Vec2,
    cell_type: MapCellType
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

#[derive(Component)]
struct ComputeMapChunkTask(Task<MapChunkResult>);


#[derive(Resource, Clone)]
struct Map {
    width: i64,
    height: i64,
    cells: HashMap<I64Vec2, MapCellType>,
    gen_status: MapGenerationStatus,
    iteration: i32,
    conflicts_count: i64
}
#[derive(Resource)]
struct MapGenSystem(SystemId);

#[derive(Resource, Clone, Eq, PartialEq)]
enum MapGenerationStatus {
    Init,
    Generating,
    Generated
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum ProcGameplaySet {
    MapGeneration,
    Player,
    Enemies,
}

fn main() {
    let (width, height) = (MAPWIDTH, MAPHEIGHT);
    let world_width = width / CELLSIZE;
    let world_height = height / CELLSIZE;
    let map = Map {
        width: world_width as i64,
        height: world_height as i64,
        cells: {
            let mut _cells:HashMap<I64Vec2, MapCellType> = HashMap::new();
            // Map need a way to query current cell value from x,y
            for x in 0..world_width as i64 {
                for y in 0..world_height as i64 {
                    _cells.insert(I64Vec2::new(x, y), MapCellType::Undeclared);
                }
            };
            _cells
        },
        gen_status: MapGenerationStatus::Init,
        iteration: 0,
        conflicts_count: 1000
    };
    App::new()
        .insert_resource(map)
        .insert_resource(Time::<Fixed>::from_seconds(0.01))
        .add_event::<MapGenTaskFinished>()
        .add_event::<ShouldGenMapEvent>()
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
            setup_map
        ).chain())
        .add_systems(FixedUpdate,(
            apply_map_cell_value,
            poll_gen_map_tasks,
            update_map_gen_status,
        ).in_set(ProcGameplaySet::MapGeneration))
        .add_systems(Update, (input_regenerate_map,gen_map_receive))
        .configure_sets(FixedUpdate, (
            ProcGameplaySet::MapGeneration.run_if(is_map_not_generated)
        ))
        .run();
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
    for (coord, cell_type) in map.cells.iter() {
        let color = cell_type.color();
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::splat(CELLSIZE as f32))).into(),
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                (coord.x*CELLSIZE as i64) as f32,
                (coord.y*CELLSIZE as i64) as f32,
                0.0,
            ),
            ..default()
        }, CellComponent{
            coord:*coord,
            cell_type:*cell_type
    }));
    }
    let map_gen_systemid = commands.register_one_shot_system(gen_map_chunk);
    commands.insert_resource(MapGenSystem(map_gen_systemid));
    commands.run_system(map_gen_systemid);
}

fn is_map_not_generated(
    map: Res<Map>
)-> bool {
    map.gen_status != MapGenerationStatus::Generated
}

fn update_map_gen_status(
    mut gen_map_event: EventWriter<ShouldGenMapEvent>,
    mut events: EventReader<MapGenTaskFinished>,
    mut map: ResMut<Map>,
) {
    for _ in events.read() {
        if map.gen_status == MapGenerationStatus::Generating
        {
            if map.conflicts_count > 0
            {
                map.iteration += 1;
                info!("MAPGEN:: Map iteration {} with {} conflict cells!\n", map.iteration, map.conflicts_count);
                gen_map_event.send_default();
            } else {
                map.gen_status = MapGenerationStatus::Generated;
                info!("MAPGEN:: Map generation finished after {} iteration!\n", map.iteration);
            }
        }
    };
}

fn check_conflicts(
    cell_coord: I64Vec2,
    map: &HashMap<I64Vec2, MapCellType>,
    map_size: I64Vec2
) -> usize {
    let mut conflicts = 0;
    let search_range = NCELLSEARCHRANGE as i64;
    if let Some(cell_type) = map.get(&cell_coord)
    {
        for dx in -search_range..search_range {
            for dy in -search_range..search_range {
                let tx = (dx + cell_coord.x + map_size.x) % map_size.x;
                let ty = (dy + cell_coord.y + map_size.y) % map_size.y;
                let check_coord = I64Vec2::new(tx, ty);
                if let Some(checkcell) = map.get(&check_coord)
                {
                    conflicts += cell_type.check_conflict(*checkcell);
                }
            }
        }
    }
    conflicts
}

fn gen_map_receive(
    mut commands: Commands,
    map_gen_system: Res<MapGenSystem>,
    mut events: EventReader<ShouldGenMapEvent>
) {
    for _ in events.read() {
        commands.run_system(map_gen_system.0)
    }
}

fn gen_map_chunk (
    mut commands: Commands,
    mut map: ResMut<Map>
) {
    let task_pool = AsyncComputeTaskPool::get();
    let map_size = I64Vec2::new(map.width, map.height);
    map.gen_status = MapGenerationStatus::Generating;
    let mut map_clone = map.cells.clone();
    let task = task_pool.spawn(async move {
        
        let conflicts = find_least_cell_conflict(&mut map_clone, map_size);
        MapChunkResult {
            map: map_clone,
            conflicts_count: conflicts
        }
    });
    commands.spawn(ComputeMapChunkTask(task));
}

#[allow(unused_assignments)]
fn find_least_cell_conflict(
    map: &mut HashMap<I64Vec2, MapCellType>,
    map_size: I64Vec2
) -> i64{
    let mut conflict_count = 0;
    for _ in 0..map.len()
    {
        let x = rand::thread_rng().gen_range(0..map_size.x);
        let y = rand::thread_rng().gen_range(0..map_size.y);
        let cell_coord = I64Vec2::new(x,y);
        let tries = ITERATION;
        debug!("MAPGEN:: find least conflict for {cell_coord} ");
        if map.contains_key(&cell_coord)
        {
            let selected_cell = map[&cell_coord];
            
            let conflicts = check_conflicts( cell_coord, &map, map_size) as i64;
        
            if conflicts > 0 || selected_cell == MapCellType::Undeclared
            {
                conflict_count += 1;
                let mut best_type = MapCellType::Undeclared;
                let mut least_conflicts: i64 = 100;
                let mut temp_terrain = MapCellType::Undeclared;
                let mut temp_conflicts: i64 = 0;
                for _ in 0..tries {
                    let temp_terrain_num = rand::thread_rng().gen_range(1..get_cell_num_type());
                    temp_terrain = MapCellType::from_repr(temp_terrain_num as usize).unwrap_or_default();
                    map.insert(cell_coord, temp_terrain);
                    temp_conflicts = check_conflicts(cell_coord, &map, map_size) as i64;
                    if temp_conflicts < least_conflicts 
                    {
                        best_type = temp_terrain;
                        least_conflicts = temp_conflicts;
                    }
                };
                map.insert(cell_coord, best_type);
                debug!(":: found best type {:?}\n", best_type);
            } 
        }
    }
    conflict_count
}

fn apply_map_cell_value(
    map: ResMut<Map>,
    mut events: EventReader<MapGenTaskFinished>,
    mut query: Query<(&mut CellComponent, &Handle<ColorMaterial>,)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in events.read() {
        query.iter_mut().for_each( |(mut cell_comp, material_handle)| {
            if let Some(cell_type) = map.cells.get(&cell_comp.coord)
            {
                cell_comp.cell_type = *cell_type;
            }
            if let Some(material) = color_materials.get_mut(material_handle)
            {
                material.color = cell_comp.cell_type.color();
                debug!("MAPGEN:: update {} color to {:?}\n", &cell_comp.coord, cell_comp.cell_type.color());
            }
            
        });
    };
}

fn poll_gen_map_tasks(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut events: EventWriter<MapGenTaskFinished>,
    mut tasks: Query<(Entity, &mut ComputeMapChunkTask)>
) {
    tasks.iter_mut().for_each (|(entity, mut task)| {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).despawn();
            map.cells = result.map;
            map.conflicts_count = result.conflicts_count;
            events.send_default();
        }
    })

}

fn input_regenerate_map(
    mut gen_map_events: EventWriter<ShouldGenMapEvent>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut map: ResMut<Map>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if map.gen_status == MapGenerationStatus::Generated
        {
            map.gen_status = MapGenerationStatus::Init;
            map.iteration = 0;
            map.conflicts_count = 1000;
            map.cells = {
                let mut _cells:HashMap<I64Vec2, MapCellType> = HashMap::new();
                // Map need a way to query current cell value from x,y
                for x in 0..map.width {
                    for y in 0..map.height {
                        _cells.insert(I64Vec2::new(x,y), MapCellType::Undeclared);
                    }
                };
                _cells
            };
            info!("MAPGEN:: Regenerating Map ...");
            gen_map_events.send_default();
        }
    }
}