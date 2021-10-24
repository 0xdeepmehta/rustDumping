use std::fmt::Debug;
use std::str::FromStr;
use regex::Regex;
use super::utils::ParseError;

/// Commands steer the ship
///
/// They consist of an operation and a parameter "value".
#[derive(Debug)]
struct Command {
    operation: char,
    value: i32,
}

/// Parse a Command from the text input
impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(\w)(\d+)$").unwrap();
        }

        let cap = RE.captures(s).unwrap();
        let operation = cap[1].chars().next().ok_or(ParseError::new(&format!("Unable to parse input: '{}'", s)))?;
        let value = cap[2].parse::<i32>()?;

        Ok(Command { operation, value })
    }
}

/// A CommandStrategy is the interface for the actual implementation of each
/// command or group of commands. There are four command groups
///
/// CompassDirectMovement
/// Implements the NESW commands for part 1 that move the ship directly
/// without considering the ship's speed.
///
/// CompassSpeedAdjustment
/// Implements the NESW commands for part 2 that modifies the ships speed.
///
/// ForwardMovement
/// Imlements the F command. This one is identical for both parts.
///
/// Rotation
/// Implements the L and R commands by modifying the speed.
trait CommandStrategy {
    fn supports_command(&self, command: &Command) -> bool;

    fn apply(&self, command: &Command, ship: &Ship) -> Result<Ship, ParseError>;
}

struct CompassDirectMovement {}

impl CommandStrategy for CompassDirectMovement {
    fn supports_command(&self, command: &Command) -> bool {
        match command.operation {
            'N' | 'E' | 'S' | 'W' => true,
            _ => false,
        }
    }

    fn apply(&self, command: &Command, ship: &Ship) -> Result<Ship, ParseError> {
        let mut ship = ship.clone();

        let delta = Position::delta(command.operation, command.value)?;
        ship.position = ship.position.translate(&delta);

        Ok(ship)
    }
}

struct CompassSpeedAdjustment {}

impl CommandStrategy for CompassSpeedAdjustment {
    fn supports_command(&self, command: &Command) -> bool {
        match command.operation {
            'N' | 'E' | 'S' | 'W' => true,
            _ => false,
        }
    }

    fn apply(&self, command: &Command, ship: &Ship) -> Result<Ship, ParseError> {
        let mut ship = ship.clone();

        let delta = Position::delta(command.operation, command.value)?;
        ship.speed = ship.speed.translate(&delta);

        Ok(ship)
    }
}
struct ForwardMovement {}

impl CommandStrategy for ForwardMovement {
    fn supports_command(&self, command: &Command) -> bool {
        command.operation == 'F'
    }

    fn apply(&self, command: &Command, ship: &Ship) -> Result<Ship, ParseError> {
        let mut ship = ship.clone();

        let delta = ship.speed.scale(command.value);
        ship.position = ship.position.translate(&delta);

        Ok(ship)
    }
}

struct Rotation {}

impl CommandStrategy for Rotation {
    fn supports_command(&self, command: &Command) -> bool {
        let direction = command.operation;
        direction == 'R' || direction == 'L'
    }

    fn apply(&self, command: &Command, ship: &Ship) -> Result<Ship, ParseError> {
        let mut ship = ship.clone();

        ship.speed = ship.speed.rotate(command.operation, command.value)?;
        Ok(ship)
    }
}

/// Marks a point on a 2d plane and provides some basic operations on that
/// point, e.g. scaling, rotation and translation.
#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn delta(direction: char, distance: i32) -> Result<Position, ParseError> {
        let (x, y) = match direction {
            'N' => (0, distance),
            'E' => (distance, 0),
            'S' => (0, -distance),
            'W' => (-distance, 0),
            e => Err(ParseError::new(&format!("Invalid direction: '{}'", e)))?
        };

        Ok(Position::new(x, y))
    }

    pub fn scale(&self, factor: i32) -> Position {
        let mut position = self.clone();

        position.x *= factor;
        position.y *= factor;

        position
    }

    pub fn rotate(&self, direction: char, angle: i32) -> Result<Position, ParseError> {
        let x = self.x;
        let y = self.y;

        let (new_x, new_y) = match (direction, angle) {
            ('L', 90) => (-y, x),
            ('L', 180) => (-x, -y),
            ('L', 270) => (y, -x),

            ('R', 90) => (y, -x),
            ('R', 180) => (-x, -y),
            ('R', 270) => (-y, x),

            (d, a) => Err(ParseError::new(&format!("Invalid rotation direction ('{}') or angle '{}'", d, a)))?,
        };

        Ok(Position::new(new_x, new_y))
    }

    pub fn translate(&self, delta: &Position) -> Position {
        Position { x: self.x + delta.x, y: self.y + delta.y }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Ship {
    position: Position,
    speed: Position,
}

impl Ship {
    pub fn init(speed: Position) -> Self {
        Ship {
            position: Position::new(0, 0),
            speed,
        }
    }
}

/// The navigator holds the implementation for the commands and applies them to
/// the ship based on the input.
struct Navigator {
    strategies: Vec<Box<dyn CommandStrategy>>,
}

impl Navigator {
    fn apply_command(&self, ship: Ship, command: &Command) -> Result<Ship, ParseError> {
        let strategy = self.strategies.iter()
            .filter(|s| s.supports_command(command))
            .next()
            .ok_or(ParseError::new(&format!("Could not find strategy for command '{:?}'", command)))?;

        strategy.apply(command, &ship)
    }
}

fn parse_input() -> Result<Vec<Command>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| Command::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let strategies: Vec<Box<dyn CommandStrategy>> = vec![
        Box::new(CompassDirectMovement {}),
        Box::new(ForwardMovement {}),
        Box::new(Rotation {})
    ];
    let navigator = Navigator { strategies };
    let initial_speed = Position::new(1, 0);
    let ship = Ship::init(initial_speed);

    let commands = parse_input()?;

    let destination = commands.into_iter()
        .try_fold(ship, |ship, command| navigator.apply_command(ship, &command))?;

    println!("12/1: manhattan distance: {}", destination.position.manhattan());

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let strategies: Vec<Box<dyn CommandStrategy>> = vec![
        Box::new(CompassSpeedAdjustment {}),
        Box::new(ForwardMovement {}),
        Box::new(Rotation {})
    ];
    let navigator = Navigator { strategies };
    let initial_speed = Position::new(10, 1);
    let ship = Ship::init(initial_speed);

    let commands = parse_input()?;

    let destination = commands.into_iter()
        .try_fold(ship, |ship, command| navigator.apply_command(ship, &command))?;

    println!("12/2: manhattan distance: {}", destination.position.manhattan());

    Ok(())
}
