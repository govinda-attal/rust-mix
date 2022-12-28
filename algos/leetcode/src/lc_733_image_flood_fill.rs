pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let key = image[sr as usize][sc as usize];
    let mut pos_list = vec![(sr, sc)];

    let width = image[0].len() as i32;
    let height = image.len() as i32;

    if key == color {
        return image;
    }

    image[sr as usize][sc as usize] = color;

    while let Some(pos) = pos_list.pop() {
        let (left, right, up, down) = (
            (pos.0, 0.max(pos.1 - 1)),
            (pos.0, (width - 1).min(pos.1 + 1)),
            (0.max(pos.0 - 1), pos.1),
            ((height - 1).min(pos.0 + 1), pos.1),
        );

        for (r, c) in [left, right, up, down] {
            if image[r as usize][c as usize] == key {
                pos_list.push((r, c));
                image[r as usize][c as usize] = color;
            }
        }
    }

    image
}

#[cfg(test)]
mod test {
    use super::flood_fill;

    #[test]
    fn different_color() {
        let coloured_image = flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);

        assert_eq!(
            coloured_image,
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        )
    }

    #[test]
    fn same_color() {
        let result = flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 1);
        assert_eq!(result, vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn one_pixel() {
        let result = flood_fill(vec![vec![1, 1, 1], vec![1, 4, 0], vec![1, 0, 1]], 1, 1, 9);
        assert_eq!(result, vec![vec![1, 1, 1], vec![1, 9, 0], vec![1, 0, 1]]);
    }
}
