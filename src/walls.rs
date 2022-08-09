use std::collections::{HashMap, HashSet};

use super::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<WallBundle>("Collision", 1)
            .add_system(spawn_wall_collision.run_in_state(GameState::Game));
    }
}

#[derive(LdtkIntCell, Bundle)]
struct WallBundle {
    wall: Wall,
}

#[derive(Component, Default)]
struct Wall;

fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Rect {
        pub left: i32,
        pub right: i32,
        pub top: i32,
        pub bottom: i32,
    }

    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        if let Ok(level_entity) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(level_entity.get())
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = *levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point")
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")
                    .iter()
                    .find(|v| v.identifier == "Collision")
                    .expect("Level should have a Walls layer");

                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                for wall_rect in wall_rects {
                    commands.entity(level_entity).with_children(|builder| {
                        builder
                            .spawn()
                            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.0,
                            )))
                            .insert(RigidBody::Fixed)
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.0,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.0,
                            ));
                    });
                }
            }
        })
    }
}
