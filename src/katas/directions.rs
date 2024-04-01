#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut z: Vec<Direction> = arr.to_vec();
    for i in 0..arr.len() {
        let a = arr[i];
        if let Some(b) = arr.get(i + 1) {
            if get_inv(&a) == *b {
                z.drain(i..i + 2);
                z = dir_reduc(&z);
                break;
            }
        }
    }

    z
}

pub fn get_inv(dir: &Direction) -> Direction {
    match dir {
        Direction::South => Direction::North,
        Direction::North => Direction::South,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::*};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}
