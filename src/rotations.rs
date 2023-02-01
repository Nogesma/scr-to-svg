pub fn rotate_2d_matrix(matrix: &mut Vec<Vec<i8>>, size: usize, clockwise: bool) {
    for i in 0..size {
        for j in (i + 1)..size {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    if clockwise {
        for m in matrix {
            for j in 0..(size / 2) {
                m.swap(j, size - j - 1);
            }
        }
    } else {
        for i in 0..(size / 2) {
            for j in 0..size {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[size - i - 1][j];
                matrix[size - i - 1][j] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod test {
    #![cfg(target_arch = "wasm32")]
    extern crate wasm_bindgen_test;

    use crate::rotations::rotate_2d_matrix;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_rotate_2d_matrix() {
        for i in 0..10 {
            let mut t = (0..i).map(|a| (0..i).map(|b| (a + b)).collect()).collect();
            rotate_2d_matrix(i, &mut t);
        }
    }
}
