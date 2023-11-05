from typing import Any, List, Optional


class CartesianCoordinate(object):
    x: int
    y: int

    def __init__(cls, x: int, y: int) -> None: ...
    def to_cube(self) -> CubeCoordinates: ...
    def to_index(self) -> Optional[int]: ...
    def from_index(index: int) -> CartesianCoordinate: ...


class CubeCoordinates(object):
    q: int
    r: int
    s: int

    def __init__(cls, q: int, r: int) -> None: ...
    def coordinates(self) -> List[int]: ...
    def x(self) -> int: ...
    def y(self) -> int: ...
    def to_cartesian(self) -> CartesianCoordinate: ...
    def times(self, count: int) -> CubeCoordinates: ...
    def plus(self, other: CubeCoordinates) -> CubeCoordinates: ...
    def minus(self, other: CubeCoordinates) -> CubeCoordinates: ...
    def unary_minus(self) -> CubeCoordinates: ...
    def rotated_by(self, turns: int) -> CubeCoordinates: ...
    def distance_to(self, other: CubeCoordinates) -> int: ...
    def turn_count_to(self, target: CubeDirection) -> int: ...


class CubeDirection:
    Right = 0
    DownRight = 1
    DownLeft = 2
    Left = 3
    UpLeft = 4
    UpRight = 5

    def vector(self) -> CubeCoordinates: ...
    def angle(self) -> int: ...
    def with_neighbors(self) -> List[CubeDirection]: ...
    def opposite(self) -> CubeDirection: ...
    def turn_count_to(self, target: CubeDirection) -> int: ...
    def rotated_by(self, turns: int) -> CubeDirection: ...
    def ordinal(self) -> int: ...


class Move:
    actions: List[Accelerate | Advance | Push | Turn]

    def __init__(self, actions: List[Accelerate |
                 Advance | Push | Turn]) -> None: ...


class TeamEnum:
    One = 0
    Two = 1


class Ship:
    team: TeamEnum
    moves: List[Move]
    position: CubeCoordinates
    direction: CubeDirection
    speed: int
    coal: int
    passengers: int
    free_turns: int
    points: int
    free_acc: int
    movement: int

    def __init__(self, position: CubeCoordinates, team: TeamEnum,
                 direction: Optional[CubeDirection] = None,
                 speed: Optional[int] = None,
                 coal: Optional[int] = None,
                 passengers: Optional[int] = None,
                 free_turns: Optional[int] = None,
                 points: Optional[int] = None): ...

    def can_turn(self) -> bool: ...
    def max_acc(self) -> int: ...
    def accelerate_by(self, diff: int) -> None: ...
    def read_resolve(self) -> None: ...
    def __str__(self) -> str: ...


class AccelerationProblem:
    ZeroAcc: Any
    AboveMaxSpeed: Any
    BelowMinSpeed: Any
    InsufficientCoal: Any
    OnSandbank: Any

    def message(self) -> str: ...


class Accelerate:
    acc: int

    def __init__(self, acc: int) -> None: ...
    def perform(self, state: Any) -> GameState | AccelerationProblem: ...
    def accelerate(self, ship: Any) -> Ship: ...
    def __str__(self) -> str: ...


class AdvanceProblem:
    MovementPointsMissing: Any
    InsufficientPush: Any
    InvalidDistance: Any
    ShipAlreadyInTarget: Any
    FieldIsBlocked: Any
    MoveEndOnSandbank: Any

    def message(self) -> str: ...


class Advance:
    distance: int

    def __init__(self, distance: int) -> None: ...
    def perform(self, state: GameState) -> GameState | AdvanceProblem: ...


class PushProblem:
    MovementPointsMissing: Any
    SameFieldPush: Any
    InvalidFieldPush: Any
    BlockedFieldPush: Any
    SandbankPush: Any
    BackwardPushingRestricted: Any

    def message(self) -> str: ...


class Push:
    direction: CubeDirection

    def __init__(self, direction: CubeDirection) -> None: ...
    def perform(self, state: GameState) -> GameState | PushProblem: ...


class TurnProblem():
    RotationOnSandbankNotAllowed: Any
    NotEnoughCoalForRotation: Any
    RotationOnNonExistingField: Any

    def message(self) -> str: ...


class Turn:
    direction: CubeDirection

    def __init__(self, direction: CubeDirection) -> None: ...
    def perform(self, state: GameState) -> GameState | TurnProblem: ...
    def coal_cost(self, ship: Ship) -> int: ...


class Passenger:
    direction: CubeDirection
    passenger: int


class FieldType:
    Water: FieldType
    Island: FieldType
    Passenger: FieldType
    Goal: FieldType
    Sandbank: FieldType


class Field:
    field_type: FieldType
    passenger: Optional[Passenger]


def __init__(field_type: FieldType,
             passenger: Optional[Passenger]) -> None: ...


def is_empty(self: Field) -> bool: ...
def is_field_type(self: Field, field_type: FieldType) -> bool: ...


class Segment:
    direction: CubeDirection
    center: CubeCoordinates
    fields: List[List[Field]]

    def tip(self) -> CubeCoordinates: ...
    def get(self, coordinates: CubeCoordinates) -> Optional[Field]: ...

    def local_to_global(
        self, coordinates: CubeCoordinates) -> CubeCoordinates: ...
    def global_to_local(
        self, coordinates: CubeCoordinates) -> CubeCoordinates: ...

    def contains(self, coordinates: CubeCoordinates) -> bool: ...


class Board:
    segments: List[Segment]
    next_direction: CubeDirection

    def __init__(self, segments: List[Segment],
                 next_direction: CubeDirection) -> None: ...

    def get_segment(self, index: int) -> Optional[Segment]: ...
    def get(self, coords: CubeCoordinates) -> Optional[Field]: ...
    def does_field_have_current(self, coords: CubeCoordinates) -> bool: ...

    def get_field_current_direction(
        self, coords: CubeCoordinates) -> Optional[CubeDirection]: ...

    def get_field_in_direction(
        self, direction: CubeDirection, coords: CubeCoordinates) -> Optional[Field]: ...

    def get_coordinate_by_index(
        self, segment_index: int, x_index: int, y_index: int) -> CubeCoordinates: ...
    def segment_distance(self, coordinate1: CubeCoordinates,
                         coordinate2: CubeCoordinates) -> int: ...

    def segment_index(self, coordinate: CubeCoordinates) -> Optional[int]: ...

    def find_segment(
        self, coordinate: CubeCoordinates) -> Optional[Segment]: ...
    def neighboring_fields(
        self, coords: CubeCoordinates) -> List[Optional[Field]]: ...

    def effective_speed(self, ship: Ship) -> int: ...
    def pickup_passenger(self, state: GameState) -> GameState: ...

    def pickup_passenger_at_position(
        self, pos: CubeCoordinates) -> Optional[Field]: ...
    def find_nearest_field_types(self, start_coordinates: CubeCoordinates,
                                 field_type: FieldType) -> List[CubeCoordinates]: ...


class TeamPoints:
    ship_points: int
    coal_points: int
    finish_points: int


class GameState:
    board: Board
    turn: int
    team_one: Ship
    team_two: Ship
    last_move: Optional[Move]

    def __init__(self, board: Board, turn: int, team_one: Ship,
                 team_two: Ship, last_move: Optional[Move]) -> None: ...

    def current_ship(self) -> Ship: ...
    def other_ship(self) -> Ship: ...
    def determine_ahead_team(self) -> Ship: ...
    def ship_advance_points(self, ship: Ship) -> int: ...
    def calculate_points(self, ship: Ship) -> int: ...
    def is_current_ship_on_current(self) -> bool: ...
    def perform_move(self, move: Move) -> GameState: ...
    def advance_turn(self, state: GameState) -> GameState: ...
    def get_simple_moves(self, max_coal: int) -> list: ...
    def get_actions(self, rank: int, max_coal: int) -> list: ...
    def must_push(self) -> bool: ...
    def get_pushes(self) -> list: ...
    def get_pushes_from(self, position: CubeCoordinates,
                        incoming_direction: CubeDirection) -> list: ...

    def get_turns(self, max_coal: int) -> list: ...
    def get_advances(self) -> list: ...
    def check_sandbank_advances(self, ship: Ship) -> list: ...
    def check_advance_limit(self, start: CubeCoordinates,
                            direction: CubeDirection, max_movement_points: int) -> list: ...

    def get_accelerations(self, max_coal: int) -> list: ...
    def can_move(self) -> bool: ...
    def is_over(self) -> bool: ...
    def is_winner(self, ship: Ship) -> bool: ...
    def get_points_for_team(self, ship: Ship) -> TeamPoints: ...
