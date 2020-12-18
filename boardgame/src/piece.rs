use crate::player::Player;
use amethyst::ecs::{Component, DenseVecStorage};
use nalgebra::base::Vector3;

pub struct Piece {
    pub x: u32,
    pub y: u32,
    pub size: f32,
    pub player: Player,
    pub piece_type: PieceType,
}
impl Piece {
    pub fn new(x: u32, y: u32, size: f32, player: Player, piece_type: PieceType) -> Piece {
        Piece {
            x,
            y,
            size,
            player,
            piece_type,
        }
    }

    pub fn get_self_xy(&self) -> Vector3<f32> {
        Vector3::new(
            self.x as f32 * self.size + self.size / 2.,
            self.y as f32 * self.size + self.size / 2.,
            1.,
        )
    }

    pub fn is_clicked(&self, x_click: f32, y_click: f32) -> bool {
        self.xy_to_grid(x_click, y_click) == (self.x, self.y)
    }
    pub fn delta(&self, x_click: f32, y_click: f32) -> (i32, i32) {
        let (x_click, y_click) = self.xy_to_grid(x_click, y_click);
        (
            (self.x as i32 - x_click as i32) as i32,
            (self.y as i32 - y_click as i32) as i32,
        )
    }

    pub fn xy_to_grid(&self, x: f32, y: f32) -> (u32, u32) {
        ((x / self.size) as u32, y as u32 / self.size as u32)
    }

    pub fn move_by(&mut self, x_click: f32, y_click: f32) {
        let (x_new, y_new) = self.xy_to_grid(x_click, y_click);
        self.x = x_new;
        self.y = y_new;
    }
}
impl Component for Piece {
    type Storage = DenseVecStorage<Self>;
}
#[derive(Copy, Clone)]
pub enum Status {
    None,
    Selected,
}
impl Component for Status {
    type Storage = DenseVecStorage<Self>;
}
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    KNIGHT,
    BISHOP,
    PAWN,
}
