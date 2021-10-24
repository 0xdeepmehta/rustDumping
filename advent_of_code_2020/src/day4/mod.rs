use super::utils::ParseError;
use regex::Regex;

fn get_validator(key: &str) -> Box<dyn FieldValidator> {
    match key {
        "byr" => Box::new(YearValidator { min: 1920, max: 2002 }),
        "iyr" => Box::new(YearValidator { min: 2010, max: 2020 }),
        "eyr" => Box::new(YearValidator { min: 2020, max: 2030 }),
        "hgt" => Box::new(HeightValidator {}),
        "hcl" => Box::new(PatternValidator { regex: Regex::new(r"^\#([0-9a-f]{6})$").unwrap() }),
        "ecl" => Box::new(PatternValidator { regex: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap() }),
        "pid" => Box::new(PatternValidator { regex: Regex::new(r"^\d{9}$").unwrap() }),
        "cid" => Box::new(EmptyValidator { value: true }),
        _ => Box::new(EmptyValidator { value: false }),
    }
}

struct Field<'a> {
    identifier: &'a str,
    value: &'a str,
}

impl<'a> Field<'a> {
    pub fn new(
        identifier: &'a str,
        value: &'a str) -> Self {

        Field { identifier, value }
    }

    pub fn is_valid(&'a self) -> bool {
        // TODO: optimize this, we should not instantiate the validators every
        // time
        let validator = get_validator(self.identifier);
        validator.is_valid(self)
    }
}

trait FieldValidator {
    fn is_valid(&self, field: &Field) -> bool;
}

struct EmptyValidator {
    value: bool,
}

impl FieldValidator for EmptyValidator {
    fn is_valid(&self, _: &Field) -> bool {
        self.value
    }
}

struct YearValidator {
    min: i16,
    max: i16,
}

impl FieldValidator for YearValidator {
    fn is_valid(&self, field: &Field) -> bool {
        field.value.parse::<i16>()
            .map(|v| v >= self.min && v <= self.max)
            .unwrap_or(false)
    }
}

struct HeightValidator {
}

impl FieldValidator for HeightValidator {
    fn is_valid(&self, field: &Field) -> bool {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(\d*)(cm|in)").unwrap();
        }

        if !RE.is_match(&field.value) {
            return false;
        }

        RE.captures(&field.value)
            .map(|cap| {
                let unit = &cap[2];
                // TODO: Get rid of this unwrap
                let size = cap[1].parse::<u32>().unwrap();

                if unit == "cm" && size >= 150 && size <= 193 {
                    true
                } else if unit == "in" && size >= 59 && size <= 76 {
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

struct PatternValidator {
    regex: Regex,
}

impl FieldValidator for PatternValidator {
    fn is_valid(&self, field: &Field) -> bool {
        self.regex.is_match(field.value)
    }
}

struct Passport<'a> {
    fields: Vec<Field<'a>>,
}

fn parse_field<'a>(field: &'a str) -> Result<Field<'a>, ParseError> {
    let key_value = field.split(':').collect::<Vec<_>>();

    if key_value.len() != 2 {
        Err(ParseError::new("foo"))
    } else {
        Ok(Field::new(key_value[0], key_value[1]))
    }
}

impl<'a> Passport<'a> {
    fn from_str(line: &'a str) -> Result<Passport<'a>, ParseError> {
        let fields = line.split(|c| c == ' ' || c == '\n')
            .filter(|v| !v.is_empty())
            .map(parse_field)
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Passport { fields })
    }
}

impl<'a> Passport<'a> {
    fn has_required_fields(&self) -> bool {
        let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_fields.iter()
            .all(|&k| self.fields.iter().any(|f| f.identifier == k))
    }

    fn is_valid(&self) -> bool {
        self.fields
            .iter()
            .all(|v| v.is_valid())
    }
}

fn parse_input<'a>() -> Result<Vec<Passport<'a>>, ParseError> {
    let input = include_str!("./data/input.txt");

    input.split("\n\n")
        .filter(|v| !v.is_empty())
        .map(|v| Passport::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<usize, ParseError> {
    let input = parse_input()?;

    let solution = input.iter()
        .filter(|p| p.has_required_fields())
        .count();

    println!("4/1: # of 'valid' passports: {}", solution);

    Ok(solution)
}

pub fn problem2() -> Result<usize, ParseError> {
    let input = parse_input()?;

    let solution = input.iter()
        .filter(|p| p.has_required_fields())
        .filter(|p| p.is_valid())
        .count();

    println!("4/2: # of 'valid' passports: {}", solution);

    Ok(solution)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn valid_expiration_year_can_be_validated() {
        let field = Field::new("eyr", "2020");

        assert_eq!(true, field.is_valid());
    }

    #[test]
    pub fn invalid_expiration_year_can_be_validated() {
        let field = Field::new("eyr", "2015");

        assert_eq!(false, field.is_valid());
    }
}
