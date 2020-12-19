use array2d::{Array2D, Error};

pub struct Water {
    current_state: Array2D<u8>,
    prev_state: Array2D<u8>,
    dampening_factor: f32
}

impl Water {
    pub fn new(x_size: usize, y_size: usize, dampening_factor: f32) -> Water {
        Water {
            current_state: Array2D::filled_with(0, x_size, y_size),
            prev_state: Array2D::filled_with(0, x_size, y_size),
            dampening_factor
        }
    }

    pub fn touch(&mut self, x_pos: usize, y_pos: usize) {
        self.current_state[(x_pos, y_pos)] = std::u8::MAX;
    }

    pub fn update(&mut self) {

    }
}

fn smooth(matrix: &mut Array2D<u8>) {
    let x_max = matrix.row_len()-1;
    let y_max = matrix.column_len()-1;
    for x_pos in 1..x_max {
        for y_pos in 1..y_max {
            matrix[(x_pos, y_pos)] = (
                matrix[(x_pos - 1, y_pos)] + matrix[(x_pos, y_pos - 1)] + matrix[(x_pos + 1, y_pos)] + matrix[(x_pos, y_pos + 1)]
            ) / 4;
        }
    }
}