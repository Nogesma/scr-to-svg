use ndarray::{Array2, Axis};

pub fn rotate_2d_matrix<T>(matrix: &mut Array2<T>, clockwise: bool) -> Array2<T>
where
    T: Clone,
{
    matrix.swap_axes(0, 1);

    let mut it = matrix.axis_iter_mut(Axis(clockwise as usize));

    while it.len() > 1 {
        ndarray::Zip::from(it.next().unwrap())
            .and(it.next_back().unwrap())
            .for_each(std::mem::swap);
    }

    matrix.to_owned()
}

pub fn get_line_intersection(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
) -> (f64, f64) {
    (
        det(det(x1, y1, x2, y2), x1 - x2, det(x3, y3, x4, y4), x3 - x4)
            / det(x1 - x2, y1 - y2, x3 - x4, y3 - y4),
        det(det(x1, y1, x2, y2), y1 - y2, det(x3, y3, x4, y4), y3 - y4)
            / det(x1 - x2, y1 - y2, x3 - x4, y3 - y4),
    )
}

fn det(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{arr2, Array};

    #[test]
    fn rotation_3x3() {
        {
            let mut a = Array::from_shape_vec((3, 3), (1..=9).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, false),
                arr2(&[[3, 6, 9], [2, 5, 8], [1, 4, 7]]),
            );
        }
        {
            let mut a = Array::from_shape_vec((3, 3), (1..=9).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, true),
                arr2(&[[7, 4, 1], [8, 5, 2], [9, 6, 3]]),
            );
        }
    }

    #[test]
    fn rotation_2x2() {
        {
            let mut a = Array::from_shape_vec((2, 2), (1..=4).collect()).unwrap();

            assert_eq!(rotate_2d_matrix(&mut a, false), arr2(&[[2, 4], [1, 3]]),);
        }
        {
            let mut a = Array::from_shape_vec((2, 2), (1..=4).collect()).unwrap();

            assert_eq!(rotate_2d_matrix(&mut a, true), arr2(&[[3, 1], [4, 2]]),);
        }
    }

    #[test]
    fn rotation_4x4() {
        {
            let mut a = Array::from_shape_vec((4, 4), (1..=16).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, false),
                arr2(&[
                    [4, 8, 12, 16],
                    [3, 7, 11, 15],
                    [2, 6, 10, 14],
                    [1, 5, 9, 13]
                ]),
            );
        }
        {
            let mut a = Array::from_shape_vec((4, 4), (1..=16).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, true),
                arr2(&[
                    [13, 9, 5, 1],
                    [14, 10, 6, 2],
                    [15, 11, 7, 3],
                    [16, 12, 8, 4]
                ]),
            );
        }
    }

    #[test]
    fn rotation_5x5() {
        {
            let mut a = Array::from_shape_vec((5, 5), (1..=25).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, false),
                arr2(&[
                    [5, 10, 15, 20, 25],
                    [4, 9, 14, 19, 24],
                    [3, 8, 13, 18, 23],
                    [2, 7, 12, 17, 22],
                    [1, 6, 11, 16, 21]
                ]),
            );
        }
        {
            let mut a = Array::from_shape_vec((5, 5), (1..=25).collect()).unwrap();

            assert_eq!(
                rotate_2d_matrix(&mut a, true),
                arr2(&[
                    [21, 16, 11, 6, 1],
                    [22, 17, 12, 7, 2],
                    [23, 18, 13, 8, 3],
                    [24, 19, 14, 9, 4],
                    [25, 20, 15, 10, 5]
                ]),
            );
        }
    }
}
