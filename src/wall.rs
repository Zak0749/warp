use super::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>("Walls", 1)
            .add_system(spawn_wall_collision);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<&GridCoords, Added<Wall>>,
    level_query: Query<&Handle<LdtkLevel>>,
    levels: Res<Assets<LdtkLevel>>,
) {
    for wall in wall_query.iter() {
        let level = levels
            .get(level_query.single())
            .expect("Level should be loaded by this point");

        let LayerInstance { grid_size, .. } = level
            .level
            .layer_instances
            .as_ref()
            .expect("Level asset should have layers")
            .iter()
            .find(|v| v.identifier == "Walls")
            .expect("Level should have a Walls layer");

        commands
            .spawn_bundle(TransformBundle::from(Transform {
                translation: Vec3::new(
                    ((wall.x * grid_size) + (grid_size / 2)) as f32,
                    ((wall.y * grid_size) + (grid_size / 2)) as f32,
                    10.,
                ),
                scale: Vec3::new(4.0 as f32, 4.0, 2.0),
                ..default()
            }))
            .insert(Collider::cuboid(
                (*grid_size as f32 / 4.0) - 0.1,
                (*grid_size as f32 / 4.0) - 0.1,
            ))
            .insert(RigidBody::Fixed)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Friction::coefficient(0.0));
    }
}
// pub fn spawn_wall_collision(
//     mut commands: Commands,
//     wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
//     parent_query: Query<&Parent, Without<Wall>>,
//     level_query: Query<(Entity, &Handle<LdtkLevel>)>,
//     levels: Res<Assets<LdtkLevel>>,
// ) {
//     /// Represents a wide wall that is 1 tile tall
//     /// Used to spawn wall collisions
//     #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Plate {
//         left: i32,
//         right: i32,
//     }

//     // consider where the walls are
//     // storing them as GridCoords in a HashSet for quick, easy lookup
//     let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

//     wall_query.for_each(|(&grid_coords, &Parent(parent))| {
//         // the intgrid tiles' direct parents will be bevy_ecs_tilemap chunks, not the level
//         // To get the level, you need their grandparents, which is where parent_query comes in
//         if let Ok(&Parent(level_entity)) = parent_query.get(parent) {
//             level_to_wall_locations
//                 .entry(level_entity)
//                 .or_insert(HashSet::new())
//                 .insert(grid_coords);
//         }
//     });

//     if !wall_query.is_empty() {
//         level_query.for_each(|(level_entity, level_handle)| {
//             if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
//                 let level = levels
//                     .get(level_handle)
//                     .expect("Level should be loaded by this point");

//                 let LayerInstance {
//                     c_wid: width,
//                     c_hei: height,
//                     grid_size,
//                     ..
//                 } = level
//                     .level
//                     .layer_instances
//                     .clone()
//                     .expect("Level asset should have layers")[0];

//                 // combine wall tiles into flat "plates" in each individual row
//                 let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

//                 for y in 0..height {
//                     let mut row_plates: Vec<Plate> = Vec::new();
//                     let mut plate_start = None;

//                     // + 1 to the width so the algorithm "terminates" plates that touch the right
//                     // edge
//                     for x in 0..width + 1 {
//                         match (plate_start, level_walls.contains(&GridCoords { x, y })) {
//                             (Some(s), false) => {
//                                 row_plates.push(Plate {
//                                     left: s,
//                                     right: x - 1,
//                                 });
//                                 plate_start = None;
//                             }
//                             (None, true) => plate_start = Some(x),
//                             _ => (),
//                         }
//                     }

//                     plate_stack.push(row_plates);
//                 }

//                 // combine "plates" into rectangles across multiple rows
//                 let mut wall_rects: Vec<Rect<i32>> = Vec::new();
//                 let mut previous_rects: HashMap<Plate, Rect<i32>> = HashMap::new();

//                 // an extra empty row so the algorithm "terminates" the rects that touch the top
//                 // edge
//                 plate_stack.push(Vec::new());

//                 for (y, row) in plate_stack.iter().enumerate() {
//                     let mut current_rects: HashMap<Plate, Rect<i32>> = HashMap::new();
//                     for plate in row {
//                         if let Some(previous_rect) = previous_rects.remove(plate) {
//                             current_rects.insert(
//                                 *plate,
//                                 Rect {
//                                     top: previous_rect.top + 1,
//                                     ..previous_rect
//                                 },
//                             );
//                         } else {
//                             current_rects.insert(
//                                 *plate,
//                                 Rect {
//                                     bottom: y as i32,
//                                     top: y as i32,
//                                     left: plate.left,
//                                     right: plate.right,
//                                 },
//                             );
//                         }
//                     }

//                     // Any plates that weren't removed above have terminated
//                     wall_rects.append(&mut previous_rects.values().copied().collect());
//                     previous_rects = current_rects;
//                 }

//                 // spawn colliders for every rectangle
//                 for wall_rect in wall_rects {
//                     commands
//                         .spawn()
//                         .insert(CollisionShape::Cuboid {
//                             half_extends: Vec3::new(
//                                 (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
//                                     * grid_size as f32
//                                     / 2.,
//                                 (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
//                                     * grid_size as f32
//                                     / 2.,
//                                 0.,
//                             ),
//                             border_radius: None,
//                         })
//                         .insert(RigidBody::Static)
//                         .insert(PhysicMaterial {
//                             friction: 0.1,
//                             ..Default::default()
//                         })
//                         .insert(Transform::from_xyz(
//                             (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.,
//                             (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.,
//                             0.,
//                         ))
//                         .insert(GlobalTransform::default())
//                         // Making the collider a child of the level serves two purposes:
//                         // 1. Adjusts the transforms to be relative to the level for free
//                         // 2. the colliders will be despawned automatically when levels unload
//                         .insert(Parent(level_entity));
//                 }
//             }
//         });
//     }
// }
