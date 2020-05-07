use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const TILE_HEIGHT: f32 = 1.0;
pub const TILE_WIDTH: f32 = 70.0;

#[derive(Debug)]
pub struct Farmer;

//The State trait is used to start, stop and update the game
impl SimpleState for Farmer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>> ){
        let mut world = data.world;

        initialise_camera(world);
        let sprite_sheet_handle = load_sprite_sheet(&mut world);
        world.register::<BackgroundSprite>();
        //initialise_background(world, sprite_sheet_handle.clone());
        self.initialize_sprite(&mut world, sprite_sheet_handle);

    }
}
impl Farmer {
    fn initialize_sprite(
        &mut self,
        world: &mut World,
        sprite_sheet_handle: Handle<SpriteSheet>) {

        // Move the sprite to the middle of the window
        let mut sprite_transform = Transform::default();
        sprite_transform.set_translation_xyz(TILE_WIDTH / 2., 50.0, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 81, // First sprite
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(BackgroundSprite::new(32))
            .with(sprite_transform)
            .build();
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


//name refering to save path of the spritesheet
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
 let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/tiles_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
pub struct BackgroundSprite {
    id: i32,
    width: f32,
    height: f32,
}

impl Component for BackgroundSprite {
    type Storage = DenseVecStorage<Self>;
}
impl BackgroundSprite {
    fn new(id: i32) -> BackgroundSprite {
        BackgroundSprite {
            id: id,
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
        }
    }
}
