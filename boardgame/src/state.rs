use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use nalgebra::base::Vector3;

use crate::piece::{Piece,PieceType, Status};
use crate::player::Player;
use log::info;

pub struct MyState {
    pub grid_size: (u32, u32),
    pub sprite_size: f32,
    pub grid: Vec<Vec<Player>>,
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.register::<Piece>();

        world.register::<Status>();
        // Place the camera
        self::MyState::init_camera(world, &dimensions);

        // Load our sprites and display them
        let background_sprites =
            self.load_sprites(world, "sprites/whiteblack.png", "sprites/whiteblack.ron");
        let piece_sprites =
            self.load_sprites(world, "sprites/chesspiecesarray.png", "sprites/chesspiecesarray.ron");
 
        self.init_background_sprites(world, background_sprites);
        self.init_piece_sprites(world, piece_sprites);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }
        // Keep going
        Trans::None
    }
}

impl MyState {
    /// Loads and splits the `logo.png` image asset into 3 sprites,
    /// which will then be assigned to entities for rendering them.
    ///
    /// The provided `world` is used to retrieve the resource loader.
    fn load_sprites(
        &self,
        world: &mut World,
        path_pic: &str,
        path_ron: &str,
    ) -> Handle<SpriteSheet> {
        // Load the texture for our sprites. We'll later need to
        // add a handle to this texture to our `SpriteRender`s, so
        // we need to keep a reference to it.
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                path_pic,
                //"sprites/whiteblack.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        // Load the spritesheet definition file, which contains metadata on our
        // spritesheet texture.
        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                path_ron,
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };
        sheet_handle
    }

    /// Creates an entity in the `world` for each of the provided `sprites`.
    /// They are individually placed around the center of the screen.
    fn init_background_sprites(&self, world: &mut World, spritesheethandle: Handle<SpriteSheet>) {
        //----loading grid -----
        let (grid_x, grid_y) = self.grid_size;
        // size of sprites in jpg TODO: find a way to extract it directly
        let def_spritesize = 20.;
        let scale = self.sprite_size as f32 / def_spritesize;
        let sprite_render_white = SpriteRender::new(spritesheethandle.clone(), 0);
        let sprite_render_black = SpriteRender::new(spritesheethandle.clone(), 1);

        for x in 0..grid_x / 2 {
            for y in 0..grid_y {
                let mut transform = Transform::default();
                let sgn = ((y + 1) % 2) as f32;
                transform.set_scale(Vector3::new(scale, scale, 1.));
                transform.set_translation_xyz(
                    x as f32 * (2. * self.sprite_size) + self.sprite_size * sgn,
                    y as f32 * self.sprite_size,
                    0.,
                );
                //screen centers the sprite on position
                transform.prepend_translation(Vector3::new(
                    self.sprite_size / 2.,
                    self.sprite_size / 2.,
                    0.,
                ));
                world
                    .create_entity()
                    .with(sprite_render_white.clone())
                    .with(transform)
                    .build();
                let sgn = (y % 2) as f32;

                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    x as f32 * (2. * self.sprite_size) + self.sprite_size * sgn,
                    y as f32 * self.sprite_size,
                    0.,
                );
                transform.prepend_translation(Vector3::new(
                    self.sprite_size / 2.,
                    self.sprite_size / 2.,
                    0.,
                ));
                transform.set_scale(Vector3::new(scale, scale, 1.));
                world
                    .create_entity()
                    .with(sprite_render_black.clone())
                    .with(transform)
                    .build();
            }
        }
        //-----loading pieces-------
    }

    fn init_piece_sprites(&self, world: &mut World, spritesheethandle: Handle<SpriteSheet>) {
        let def_spritesize = 60.;
        let scale = self.sprite_size as f32 / def_spritesize;
        let (p1,p2) = (Player::FIRST,Player::SECOND);
        //Rook
        let sprite_render_figure_rook = SpriteRender::new(spritesheethandle.clone(), 2);
        self.init_piece(sprite_render_figure_rook.clone(), world, &p1,(0,0), scale, PieceType::ROOK);
        self.init_piece(sprite_render_figure_rook, world, &p1,(7,0), scale, PieceType::ROOK);
        let sprite_render_figure_rook = SpriteRender::new(spritesheethandle.clone(), 8);
        self.init_piece(sprite_render_figure_rook.clone(), world, &p2,(0,7), scale, PieceType::ROOK);
        self.init_piece(sprite_render_figure_rook, world, &p2,(7,7), scale, PieceType::ROOK);
        //Knight
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 3);
        self.init_piece(sprite_render_figure.clone(), world, &p1,(1,0), scale, PieceType::KNIGHT);
        self.init_piece(sprite_render_figure, world, &p1,(6,0), scale, PieceType::KNIGHT);
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 9);
        self.init_piece(sprite_render_figure.clone(), world, &p2,(1,7), scale, PieceType::KNIGHT);
        self.init_piece(sprite_render_figure, world, &p2,(6,7), scale,PieceType::KNIGHT);
        //bishop
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 4);
        self.init_piece(sprite_render_figure.clone(), world, &p1,(2,0), scale, PieceType::BISHOP);
        self.init_piece(sprite_render_figure, world, &p1,(5,0), scale, PieceType::BISHOP);
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 10);
        self.init_piece(sprite_render_figure.clone(), world, &p2,(2,7), scale, PieceType::BISHOP);
        self.init_piece(sprite_render_figure, world, &p2,(5,7), scale, PieceType::BISHOP);
        //Queen
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 0);
        self.init_piece(sprite_render_figure, world, &p1,(4,0), scale, PieceType::QUEEN);
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 6);
        self.init_piece(sprite_render_figure, world, &p2,(4,7), scale, PieceType::QUEEN);
        //King
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 1);
        self.init_piece(sprite_render_figure, world, &p1,(3,0), scale, PieceType::KING);
        let sprite_render_figure = SpriteRender::new(spritesheethandle.clone(), 7);
        self.init_piece(sprite_render_figure, world, &p2,(3,7), scale, PieceType::KING);
        //Pawn
        let sprite_render_figure1 = SpriteRender::new(spritesheethandle.clone(), 5);
        let sprite_render_figure2 = SpriteRender::new(spritesheethandle.clone(), 11);
        for i in 0..8 {
        self.init_piece(sprite_render_figure1.clone(), world, &p1,(i,1), scale, PieceType::PAWN);
        self.init_piece(sprite_render_figure2.clone(), world, &p2,(i,6), scale, PieceType::PAWN);
        }
        
    }
    fn init_piece(&self, sprite_render_piece: SpriteRender, world: &mut World, player: &Player, pos: (u32,u32), scale: f32, piece_type: PieceType) {
            let piece = Piece::new(pos.0, pos.1, self.sprite_size, player.clone(), piece_type);

            let mut transform = Transform::default();
            transform.set_translation(piece.get_self_xy());
            transform.set_scale(Vector3::new(scale, scale, 1.));
            world
                .create_entity()
                .with(piece)
                .with(sprite_render_piece)
                .with(Status::None)
                .with(transform)
                .build();
    }

    /// Creates a camera entity in the `world`.
    ///
    /// The `dimensions` are used to center the camera in the middle
    /// of the screen, as well as make it cover the entire screen.
    fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(dimensions.width() / 2., dimensions.height() / 2., 200.0);

        world
            .create_entity()
            .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
            .with(transform)
            .build();
    }
}
