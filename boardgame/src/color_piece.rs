use crate::piece;
use crate::piece::Piece;
use crate::piece::Status;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{palette::Srgba, resources::Tint};
use amethyst::window::ScreenDimensions;

pub struct ColorPieceSystem;

impl<'s> System<'s> for ColorPieceSystem {
    type SystemData = (WriteStorage<'s, Piece>, WriteStorage<'s, Tint>);

    fn run(&mut self, (mut pieces, mut tints): Self::SystemData) {
        for (piece, tint) in (&mut pieces, &mut tints).join() {
            match piece.status {
                Status::None => {}
                Status::Selected => {
                    let mut tin = Tint(Srgba::new(0.5, 1., 1., 0.5));
                    write_storage.insert(tin)
                }
            }
        }
    }
}
