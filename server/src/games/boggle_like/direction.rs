
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn direction_vector(direction: &Direction) -> (i8, i8) {
        match direction {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    pub fn from_direction_vector(direction_vector: &(i8, i8)) -> Option<Direction> {
        match direction_vector {
            (0, -1) => Some(Direction::North),
            (1, -1) => Some(Direction::NorthEast),
            (1, 0) => Some(Direction::East),
            (1, 1) => Some(Direction::SouthEast),
            (0, 1) => Some(Direction::South),
            (-1, 1) => Some(Direction::SouthWest),
            (-1, 0) => Some(Direction::West),
            (-1, -1) => Some(Direction::NorthWest),
            (_, _) => None,
        }
    }

    pub fn next_clockwise(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }

    pub fn next_counter_clockwise(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::NorthWest,
            Direction::NorthEast => Direction::North,
            Direction::East => Direction::NorthEast,
            Direction::SouthEast => Direction::East,
            Direction::South => Direction::SouthEast,
            Direction::SouthWest => Direction::South,
            Direction::West => Direction::SouthWest,
            Direction::NorthWest => Direction::West,
        }
    }

    pub fn directions_between_exclusive_cw(
        start_exclusive: &Direction,
        end_exclusive: &Direction,
    ) -> Vec<Direction> {
        if start_exclusive == end_exclusive {
            vec![]
        } else {
            let mut directions: Vec<Direction> = vec![];
            let mut next = Direction::next_clockwise(start_exclusive);
            while &next != end_exclusive {
                directions.push(next);

                next = Direction::next_clockwise(&next);
            }

            directions
        }
    }

    pub fn directions_between_inclusive_cw(
        start_inclusive: &Direction,
        end_inclusive: &Direction,
    ) -> Vec<Direction> {
        let mut directions: Vec<Direction> = vec![*start_inclusive];
        let mut next = Direction::next_clockwise(start_inclusive);
        while &next != end_inclusive {
            directions.push(next);

            next = Direction::next_clockwise(&next);
        }
        directions.push(*end_inclusive);

        directions
    }

    pub fn list_directions_cw_from_north() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_vector() {
        let sut: Vec<(Direction, (i8, i8))> = vec![
            (Direction::North, (0, -1)),
            (Direction::NorthEast, (1, -1)),
            (Direction::East, (1, 0)),
            (Direction::SouthEast, (1, 1)),
            (Direction::South, (0, 1)),
            (Direction::SouthWest, (-1, 1)),
            (Direction::West, (-1, 0)),
            (Direction::NorthWest, (-1, -1)),
        ];

        for (direction, expected_vector) in sut {
            let actual_vector = Direction::direction_vector(&direction);

            assert_eq!(actual_vector, expected_vector);
        }
    }

    #[test]
    fn test_directions_between_inclusive_cw() {
        let expected_directions: Vec<Direction> = Direction::list_directions_cw_from_north();

        let actual_directions = Direction::directions_between_inclusive_cw(
            &Direction::North,
            &Direction::NorthWest,
        );

        assert_eq!(actual_directions, expected_directions);
    }
}