use crate::piece::Piece;
use crate::piece::Status;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::window::ScreenDimensions;

pub struct MoveSystem {
    pressed: bool,
}
impl Default for MoveSystem {
    fn default() -> Self {
        Self {
            //pressed is needed to overwrite serveral keyinputs from one keypress
            pressed: false,
        }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Piece>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, mut pieces, inp, dim): Self::SystemData) {
        let mut move_piece: Option<&Piece> = None;
        let (x, mut y) = inp.mouse_position().unwrap();
        //y from mouseinput and drawn objects are inverted.
        y = dim.height() - y;
        let mut status = Status::None;
        for (piece, transform) in (&mut pieces, &mut transforms).join() {
            let input = inp.action_is_down("press").unwrap_or(false);
            if input {
                self.pressed = true;
                println!("input!");
            }
            if self.pressed && !input {

                status = match piece.status {
                    Status::None => {
                        if piece.is_clicked(x, y) {
                            self.pressed = false;
                            Status::Selected
                        } else {
                            Status::None
                        }
                    }
                    Status::Selected => {
                        self.pressed = false;
                        if piece.is_clicked(x, y) {
                            Status::Selected
                        } else {
                            //if piece can move to position
                            if self.check(piece, x, y) {
                                move_piece = Some(&piece);
                                Status::None
                            } else {
                                Status::Selected
                            }
                        }
                    }
                };
                if mv {
                    piece.move_by(x, y);
                    //transform.prepend_translation(piece.move_by_grid(piece::move_by_xy(x,y,x_piece,y_piece)));
                    transform.set_translation(piece.get_self_xy());
                }
            }
        }
        match move_piece {

            Some(p) => {
        
                let moveable = true;
                for piece in (&mut pieces).join() {
                    if piece.xy_to_grid(x,y) == (piece.x,piece.y) {    
                        moveable = false;
                        p.status = status;
                        println!("cannot move");
                    }
            
                }
                if moveable {
                    piece.move_by(x, y);
                    //transform.prepend_translation(piece.move_by_grid(piece::move_by_xy(x,y,x_piece,y_piece)));
                    transform.set_translation(piece.get_self_xy());
                    

            }},
            None => {},
        }
    }
}
impl MoveSystem {
    fn check(&self, piece: &Piece, x: f32, y: f32) -> bool {

        let (x_del, y_del) = piece.delta(x, y);
        (x_del.abs(), y_del.abs()) == (1, 1) || x_del.abs() + y_del.abs() == 1
    }
}
