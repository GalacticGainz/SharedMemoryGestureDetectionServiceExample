use std::f64::consts::PI;
use crate::core::point::Point;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

impl Direction {
    pub(crate) fn name(self) -> String {
        String::from(
            match self {
                Direction::East => "EAST",
                Direction::Northeast => "NORTH_EAST",
                Direction::North => "NORTH",
                Direction::Northwest => "NORTH_WEST",
                Direction::West => "WEST",
                Direction::Southwest => "SOUTH_WEST",
                Direction::South => "SOUTH",
                Direction::Southeast => "SOUTH_EAST",
            }
        )
    }

    pub(crate) fn value_of(string: &str) -> Option<Self> {
        match string {
            "East"|"EAST" => Some(Direction::East),
            "Northeast"|"NORTHEAST" => Some(Direction::Northeast),
            "North"|"NORTH" => Some(Direction::North),
            "Northwest"|"NORTHWEST" => Some(Direction::Northwest),
            "West"|"WEST" => Some(Direction::West),
            "Southwest"|"SOUTHWEST" => Some(Direction::Southwest),
            "South"|"SOUTH" => Some(Direction::South),
            "Southeast"|"SOUTHEAST" => Some(Direction::Southeast),
            _ => None,
        }
    }
    pub(crate) fn opposite_direction(self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::Northeast => Direction::Southwest,
            Direction::North => Direction::South,
            Direction::Northwest => Direction::Southeast,
            Direction::West => Direction::East,
            Direction::Southwest => Direction::Northeast,
            Direction::South => Direction::North,
            Direction::Southeast => Direction::Northwest,
        }
    }

    pub(crate) fn adjacent_directions(self) -> Vec<Direction> {
        match self {
            Direction::East => vec![Direction::Southeast, Direction::East, Direction::Northeast],
            Direction::Northeast => vec![Direction::East, Direction::Northeast, Direction::North],
            Direction::North => vec![Direction::Northeast, Direction::North, Direction::Northwest],
            Direction::Northwest => vec![Direction::North, Direction::Northwest, Direction::West],
            Direction::West => vec![Direction::Northwest, Direction::West, Direction::Southwest],
            Direction::Southwest => vec![Direction::West, Direction::Southwest, Direction::South],
            Direction::South => vec![Direction::Southwest, Direction::South, Direction::Southeast],
            Direction::Southeast => vec![Direction::South, Direction::Southeast, Direction::East],
        }
    }

    pub(crate) fn direction_from_points(p1: Point, p2: Point) -> Direction {
        let alpha = Self::angle_from(p2, p1);
        Self::direction_from_alpha(alpha)
    }

    pub(crate) fn direction_from_alpha(alpha: f64) -> Direction {
        let sectors: Vec<(usize, f64)> = (0..8)
            .map(|i| (i, (360.0 / 8.0) * (i as f64 + 1.0) - (360.0 / 16.0)))
            .collect();

        sectors
            .iter()
            .find(|&&(_, angle)| angle >= alpha)
            .map_or(Direction::East, |&(i, _)| Self::from_index(i))
    }

    pub(crate) fn from_index(index: usize) -> Direction {
        match index {
            0 => Direction::East,
            1 => Direction::Northeast,
            2 => Direction::North,
            3 => Direction::Northwest,
            4 => Direction::West,
            5 => Direction::Southwest,
            6 => Direction::South,
            7 => Direction::Southeast,
            _ => Direction::East,
        }
    }

    pub(crate) fn java_ordinal(self) -> u32 {
        match self {
            Direction::East => 0,
            Direction::Northeast => 1,
            Direction::North => 2,
            Direction::Northwest => 3,
            Direction::West => 4,
            Direction::Southwest => 5,
            Direction::South => 6,
            Direction::Southeast => 7,
        }
    }

    pub(crate) fn angle_from(p2: Point, p1: Point) -> f64 {
        let dy = -(p2.y as f64) - -(p1.y as f64);
        let dx = p2.x as f64 - p1.x as f64;

        (360.0 + (dy.atan2(dx) * 180.0 / PI)) % 360.0
    }

    pub(crate) fn values() -> Vec<Self> {
        vec![
            Self::East,
            Self::Northeast,
            Self::North,
            Self::Northwest,
            Self::West,
            Self::Southwest,
            Self::South,
            Self::Southeast,
        ]
    }

}
impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::East, Direction::East) => true,
            (Direction::Northeast, Direction::Northeast) => true,
            (Direction::North, Direction::North) => true,
            (Direction::Northwest, Direction::Northwest) => true,
            (Direction::West, Direction::West) => true,
            (Direction::Southwest, Direction::Southwest) => true,
            (Direction::South, Direction::South) => true,
            (Direction::Southeast, Direction::Southeast) => true,
            _ => false,
        }
    }
}

