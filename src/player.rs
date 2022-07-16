use super::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(player_movement_system)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell_for_layer::<CollisionLayer>("Collision", 1);
    }
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default)]
struct Name {
    name: String,
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,

    collider: Collider,

    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {
        let tileset = tileset.expect("No titleset provided");
        let tile = entity_instance.tile.as_ref().expect("No tile provided");
        let tileset_definition = tileset_definition.expect("No tileset definition provided");

        println!("{:?} {:?}", tile.w, tile.h);

        PlayerBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlases.add(TextureAtlas::from_grid_with_padding(
                    tileset.clone(),
                    Vec2::new(tile.w as f32, tile.h as f32),
                    tileset_definition.c_wid as usize,
                    tileset_definition.c_hei as usize,
                    Vec2::splat(tileset_definition.spacing as f32),
                )),
                sprite: TextureAtlasSprite {
                    index: (tile.y / (tile.h + tileset_definition.spacing)) as usize
                        * tileset_definition.c_wid as usize
                        + (tile.x / (tile.w + tileset_definition.spacing)) as usize,
                    ..Default::default()
                },
                ..Default::default()
            },
            input_manager: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::W, PlayerAction::Up),
                    (KeyCode::S, PlayerAction::Down),
                    (KeyCode::A, PlayerAction::Left),
                    (KeyCode::D, PlayerAction::Right),
                    (KeyCode::Up, PlayerAction::Up),
                    (KeyCode::Down, PlayerAction::Down),
                    (KeyCode::Left, PlayerAction::Left),
                    (KeyCode::Right, PlayerAction::Right),
                ]),
            },
            collider: Collider::cuboid(tile.w as f32, tile.h as f32),
            ..default()
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

fn player_movement_system(
    mut query: Query<(&mut Transform, &ActionState<PlayerAction>), With<Player>>,
    collision_layer: Query<&PlayerColliders>,
    time: Res<Time>,
) {
    for (mut transfrom, action_state) in query.iter_mut() {
        let mut next_pos = transfrom.translation;
        if action_state.pressed(PlayerAction::Up) {
            next_pos.y += 100.0 * time.delta_seconds();
        }
        if action_state.pressed(PlayerAction::Down) {
            next_pos.y -= 100.0 * time.delta_seconds();
        }
        if action_state.pressed(PlayerAction::Right) {
            next_pos.x += 100.0 * time.delta_seconds();
        }
        if action_state.pressed(PlayerAction::Left) {
            next_pos.x -= 100.0 * time.delta_seconds();
        }
        for collision_layer in collision_layer.iter() {
            if next_pos == transfrom.translation {
                return;
            }

            if !collision_layer.positions.iter().any(|position| {
                collide(
                    next_pos,
                    Vec2::new(16.0, 16.0),
                    *position,
                    collision_layer.item_size,
                )
                .is_some()
            }) {
                transfrom.translation = next_pos
            }
        }
    }
}

#[derive(Component, Default)]
struct PlayerColliders {
    positions: Vec<Vec3>,
    item_size: Vec2,
}

#[derive(Bundle, Default)]
struct CollisionLayer {
    colliders: PlayerColliders,
}

impl LdtkIntCell for CollisionLayer {
    fn bundle_int_cell(_: IntGridCell, layer_instance: &LayerInstance) -> CollisionLayer {
        CollisionLayer {
            colliders: PlayerColliders {
                positions: layer_instance
                    .int_grid_csv
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| {
                        if val == &1 {
                            Some(Vec3::new(
                                ((idx as i32 % layer_instance.c_wid) * layer_instance.grid_size)
                                    as f32
                                    + 4.0,
                                ((layer_instance.grid_size * layer_instance.c_hei)
                                    - ((idx as i32 / layer_instance.c_wid)
                                        * layer_instance.grid_size))
                                    as f32
                                    - 4.0,
                                3.0,
                            ))
                        } else {
                            None
                        }
                    })
                    .collect(),
                item_size: Vec2::splat(layer_instance.grid_size as f32),
            },
            ..default()
        }
    }
}
