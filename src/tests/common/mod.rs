//! Общие вспомогательные функции и константы для модульных тестов.

use crate::game::state::GameState;
use crate::tetromino::bag_generator::BagGenerator;
use crate::tetromino::constants::SHAPE_COORDS;
use crate::tetromino::shape_type::ShapeType;
use crate::tetromino::tetromino_struct::Tetromino;
use crate::types::Direction;

/// Все 7 типов фигур — одна константа вместо дублирования
pub const ALL_SHAPES: [ShapeType; 7] = [
    ShapeType::T,
    ShapeType::L,
    ShapeType::J,
    ShapeType::S,
    ShapeType::Z,
    ShapeType::O,
    ShapeType::I,
];

/// Создать GameState с конкретной фигурой
pub fn state_with_shape(shape: ShapeType) -> GameState {
    let mut state = GameState::new();
    state.mutate_curr_shape(|s| {
        s.set_shape(shape);
        s.set_coords(SHAPE_COORDS[shape as usize]);
    });
    state
}

/// Двинуть фигуру к указанной границе до упора
pub fn push_to_wall(state: &mut GameState, dir: Direction) {
    for _ in 0..20 {
        match dir {
            Direction::Left => {
                if state.can_move_curr_shape_direction(Direction::Left) {
                    state.move_curr_dx(-1.0);
                } else {
                    break;
                }
            }
            Direction::Right => {
                if state.can_move_curr_shape_direction(Direction::Right) {
                    state.move_curr_dx(1.0);
                } else {
                    break;
                }
            }
            Direction::Down => {
                if state.can_move_curr_shape_direction(Direction::Down) {
                    state.move_curr_dy(1.0);
                } else {
                    break;
                }
            }
        }
    }
}

/// Создать Tetromino в центре поля с заданной фигурой
pub fn tetromino_at_center(shape: ShapeType) -> Tetromino {
    let shape_idx = shape as usize;
    Tetromino::new(
        (4.0, 0.0),
        shape,
        SHAPE_COORDS[shape_idx],
        shape_idx as u8,
    )
}

/// Создать BagGenerator и вернуть первую фигуру
pub fn valid_bag_shape() -> ShapeType {
    let mut bag = BagGenerator::new();
    bag.next_shape()
}
