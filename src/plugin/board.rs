use std::collections::VecDeque;

use pyo3::prelude::*;

use crate::plugin::coordinate::{ CartesianCoordinate, CubeCoordinates, CubeDirection };
use crate::plugin::field::{ Field, FieldType };
use crate::plugin::game_state::GameState;
use crate::plugin::segment::Segment;
use crate::plugin::ship::Ship;

#[pyclass]
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
pub struct Board {
    #[pyo3(get, set)]
    pub segments: Vec<Segment>,
    #[pyo3(get, set)]
    pub next_direction: CubeDirection,
}

#[pymethods]
impl Board {
    #[new]
    pub fn new(segments: Vec<Segment>, next_direction: CubeDirection) -> Self {
        Board {
            segments,
            next_direction,
        }
    }

    pub fn get_segment(&self, index: usize) -> Option<Segment> {
        self.segments.get(index).cloned()
    }

    pub fn segment_with_index_at(&self, coords: CubeCoordinates) -> Option<(usize, Segment)> {
        self.segments
            .iter()
            .enumerate()
            .find(|(_, segment)| { segment.contains(coords.clone()) })
            .map(|(i, s)| (i, s.clone()))
    }

    pub fn get(&self, coords: &CubeCoordinates) -> Option<Field> {
        for segment in &self.segments {
            if segment.contains(*coords) {
                return segment.get(*coords);
            }
        }
        None
    }

    pub fn does_field_have_stream(&self, coords: &CubeCoordinates) -> bool {
        self.segment_with_index_at(*coords)
            .map(|(i, s)| {
                let next_dir: CubeCoordinates = self.segments
                    .get(i + 1)
                    .map(|s| s.direction.vector())
                    .unwrap_or(self.next_direction.vector());
                [
                    s.center - s.direction.vector(),
                    s.center,
                    s.center + next_dir,
                    s.center + next_dir * 2,
                ].contains(&coords)
            })
            .unwrap_or(false)
    }

    pub fn get_field_in_direction(
        &self,
        direction: &CubeDirection,
        coords: &CubeCoordinates
    ) -> Option<Field> {
        self.get(&(coords.clone() + direction.vector()))
    }

    pub fn get_coordinate_by_index(
        &self,
        segment_index: usize,
        x_index: usize,
        y_index: usize
    ) -> CubeCoordinates {
        let coord: CubeCoordinates = CartesianCoordinate::new(
            x_index as i32,
            y_index as i32
        ).to_cube();
        self.segments[segment_index].local_to_global(coord)
    }

    pub fn segment_distance(
        &self,
        coordinate1: &CubeCoordinates,
        coordinate2: &CubeCoordinates
    ) -> i32 {
        let segment_index1 = self.segment_index(coordinate1).unwrap();
        let segment_index2 = self.segment_index(coordinate2).unwrap();
        i32::abs((segment_index1 as i32) - (segment_index2 as i32))
    }

    pub fn segment_index(&self, coordinate: &CubeCoordinates) -> Option<usize> {
        self.segments.iter().position(|segment| segment.contains(coordinate.clone()))
    }

    pub fn find_segment(&self, coordinate: &CubeCoordinates) -> Option<Segment> {
        let index = self.segment_index(coordinate)?;
        self.segments.get(index).cloned()
    }

    pub fn neighboring_fields(&self, coords: &CubeCoordinates) -> Vec<Option<Field>> {
        CubeDirection::VALUES.iter()
            .map(|direction| self.get_field_in_direction(&direction, coords))
            .collect()
    }

    pub fn neighboring_coordinates(
        &self,
        coords: &CubeCoordinates
    ) -> Vec<Option<CubeCoordinates>> {
        CubeDirection::VALUES.iter()
            .zip(
                CubeDirection::VALUES.iter().map(|direction|
                    self.get_field_in_direction(&direction, coords)
                )
            )
            .map(|(direction, field)| field.map(|_| coords.clone() + direction.vector()))
            .collect()
    }

    pub fn effective_speed(&self, ship: &Ship) -> i32 {
        let speed = ship.speed;
        if self.does_field_have_stream(&ship.position) {
            speed - 1
        } else {
            speed
        }
    }

    pub fn is_sandbank(&self, coords: &CubeCoordinates) -> bool {
        self.get(coords)
            .map(|field| field.field_type == FieldType::Sandbank)
            .unwrap_or(false)
    }

    pub fn pickup_passenger(&self, state: &GameState) -> GameState {
        let new_state: GameState = state.clone();
        let mut ship = new_state.current_ship;
        if self.effective_speed(&ship) < 2 {
            if let Some(mut field) = new_state.board.pickup_passenger_at_position(&ship.position) {
                field.passenger.as_mut().map(|passenger| {
                    passenger.passenger -= 1;
                });
                ship.passengers += 1;
            }
        }
        new_state
    }

    fn pickup_passenger_at_position(&self, pos: &CubeCoordinates) -> Option<Field> {
        CubeDirection::VALUES.iter()
            .filter_map(|direction| {
                let field = self.get_field_in_direction(direction, pos)?;
                if field.passenger.as_ref()?.passenger > 0 {
                    Some(field)
                } else {
                    None
                }
            })
            .next()
    }

    /// A function `find_nearest_field_types` to find the nearest field(s) of a specific type from a starting point in a 3D grid.
    ///
    /// # Arguments
    ///
    /// * `start_coordinates` - A CubeCoordinates object representing the starting point for the search.
    /// * `field_type` - A FieldType object representing the type of field being searched for.
    ///
    /// # Returns
    ///
    /// This function will return a Vector of CubeCoordinates corresponding to the location of the nearest field(s) of the specified type.
    ///
    /// This function uses the Breadth-First Search (BFS) algorithm to search through the grid.
    /// BFS was chosen because it perfectly suits for finding the shortest way in such kind of tasks.
    /// It starts at the `start_coordinates`, explores the nearest nodes first and moves towards the next level neighbours only when all the current level nodes are visited.
    ///
    /// It returns immediately when the distance to the current node is larger than the distance to the node in the `nearest_field_coordinates`,
    /// meaning it has passed the nearest node(s) and there is no need to continue the search.
    ///
    /// # Note
    ///
    /// This function will always return the coordinates of the nearest field(s) of the specified type, if such a field(s) exist.
    /// If multiple fields of the same type are at the same minimum distance, it returns all of them.
    /// If there isn't a field of the specified type or path to it, it will return an empty Vec.
    ///
    /// # Examples
    ///
    /// ```python
    /// from plugin import Board, CubeCoordinates, FieldType
    ///
    /// board = Board()
    /// board.find_nearest_field_types(CubeCoordinates(0, 0), FieldType.Water)
    /// ```
    ///
    pub fn find_nearest_field_types(
        &mut self,
        start_coordinates: &CubeCoordinates,
        field_type: FieldType
    ) -> Vec<CubeCoordinates> {
        let mut nearest_coordinates: Vec<CubeCoordinates> = Vec::new();
        let mut queue: VecDeque<(CubeCoordinates, i32)> = VecDeque::from(vec![(start_coordinates.clone(), 0)]);
        let mut last_distance: i32 = 0;

        while let Some((current_coords, distance)) = queue.pop_front() {
            if !nearest_coordinates.is_empty() && distance > last_distance {
                break;
            }

            last_distance = distance;

            if let Some(field) = self.get(&current_coords) {
                if field.field_type == field_type {
                    nearest_coordinates.push(current_coords.clone());
                }
            }

            self.neighboring_coordinates(&current_coords)
                .iter()
                .filter_map(|neighbor| neighbor.clone())
                .for_each(|coord| queue.push_back((coord, distance + 1)));
        }

        nearest_coordinates
    }

    pub fn pretty_print(&self) {
        for segment in &self.segments {
            for col in &segment.fields {
                for field in col {
                    match field.field_type {
                        FieldType::Water => print!("W"),
                        FieldType::Sandbank => print!("S"),
                        FieldType::Island => print!("I"),
                        FieldType::Passenger => print!("P"),
                        FieldType::Goal => print!("G"),
                    }
                }
                println!();
            }
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Board(segments={:?}, next_direction={:?})", self.segments, self.next_direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_segment() {}

    #[test]
    fn test_get() {}

    #[test]
    fn test_does_field_have_stream() {
        let mut segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ]
            ],
        }];
        let mut board: Board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 0)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 1)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(-1, 1)), false);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(1, 1)), false);

        segment = vec![Segment {
            direction: CubeDirection::DownRight,
            center: CubeCoordinates::new(0, 0),
            fields: vec![
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ]
            ],
        }];

        board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 0)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(0, 1)), true);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(-1, 1)), false);
        assert_eq!(board.does_field_have_stream(&CubeCoordinates::new(1, 1)), false);
    }

    #[test]
    fn test_get_field_in_direction() {}

    #[test]
    fn test_get_coordinate_by_index() {}

    #[test]
    fn test_segment_distance() {}

    #[test]
    fn test_segment_index() {}

    #[test]
    fn test_find_segment() {}

    #[test]
    fn test_neighboring_fields() {}

    #[test]
    fn test_effective_speed() {}

    #[test]
    fn test_get_field_current_direction() {}

    #[test]
    fn test_find_nearest_field_types() {
        let segment: Vec<Segment> = vec![Segment {
            direction: CubeDirection::Right,
            center: CubeCoordinates::new(0, 0),
            fields: vec![
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Sandbank, None),
                    Field::new(FieldType::Water, None)
                ],
                vec![
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None),
                    Field::new(FieldType::Water, None)
                ]
            ],
        }];
        let mut board: Board = Board::new(segment, CubeDirection::DownRight);

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(0, 0), FieldType::Sandbank),
            vec![CubeCoordinates::new(0, 0)]
        );

        board.segments[0].fields[1][2] = Field::new(FieldType::Water, None);

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(0, 0), FieldType::Sandbank),
            vec![
                CubeCoordinates::new(1, 0),
                CubeCoordinates::new(0, 1),
                CubeCoordinates::new(-1, 1),
                CubeCoordinates::new(-1, 0),
                CubeCoordinates::new(0, -1),
                CubeCoordinates::new(1, -1)
            ]
        );
        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(2, 0), FieldType::Sandbank),
            vec![CubeCoordinates::new(1, 0)]
        );

        assert_eq!(
            board.find_nearest_field_types(&CubeCoordinates::new(1, 0), FieldType::Water),
            vec![
                CubeCoordinates::new(2, 0),
                CubeCoordinates::new(1, 1),
                CubeCoordinates::new(0, 0),
                CubeCoordinates::new(2, -1)
            ]
        );
    }
}
