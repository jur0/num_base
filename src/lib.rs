
#[derive(Debug)]
pub struct NumString {
    digits: Vec<char>,
    base: u8,
    number: Result<u64, ()>,
}

impl NumString {
    pub fn new<T: Into<String>>(digits: T, base: u8) -> Self {
        let digits: Vec<char> = digits.into().to_lowercase().chars().collect();
        let number = digits
            .iter()
            .map(|digit| Self::digit_to_number(*digit, base))
            .collect::<Result<Vec<u8>, ()>>()
            .map(|numbers| Self::compute_number(numbers, base));

        Self {
            digits: digits,
            base: base,
            number: number,
        }
    }

    pub fn convert(&self, base: u8) -> Result<String, ()> {
        let mut number = self.number?;
        let base = base as u64;
        let mut numbers: Vec<u8> = Vec::new();
        let mut done = false;

        while !done {
            let digit = (number % base) as u8;
            numbers.push(digit);
            number = number / base;
            if number <= 0 {
                done = true;
            }
        }
        numbers
            .iter()
            .map(|n| Self::number_to_digit(*n))
            .collect::<Result<Vec<char>, ()>>()
            .map(|digits| digits.iter().rev().collect())
    }


    pub fn is_valid(&self) -> bool {
        self.number.is_ok()
    }

    pub fn number(&self) -> Result<u64, ()> {
        self.number
    }

    fn compute_number(numbers: Vec<u8>, base: u8) -> u64 {
        let mut number = 0;
        let base = base as u64;

        for digit in numbers {
            let digit = digit as u64;
            number = number * base + digit
        }
        number
    }

    fn digit_to_number(digit: char, base: u8) -> Result<u8, ()> {
        let number = match digit {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            'a' => Some(10),
            'b' => Some(11),
            'c' => Some(12),
            'd' => Some(13),
            'e' => Some(14),
            'f' => Some(15),
            'g' => Some(16),
            'h' => Some(17),
            'i' => Some(18),
            'j' => Some(19),
            'k' => Some(20),
            'l' => Some(21),
            'm' => Some(22),
            'n' => Some(23),
            'o' => Some(24),
            'p' => Some(25),
            'q' => Some(26),
            'r' => Some(27),
            's' => Some(28),
            't' => Some(29),
            'u' => Some(30),
            'v' => Some(31),
            'w' => Some(32),
            'x' => Some(33),
            'y' => Some(34),
            'z' => Some(35),
            _ => None,
        };
        match number {
            Some(n) => if n < base {
                Ok(n)
            } else {
                Err(())
            },
            None => Err(()),
        }
    }

    fn number_to_digit(number: u8) -> Result<char, ()> {
        let digit = match number {
            0 => Some('0'),
            1 => Some('1'),
            2 => Some('2'),
            3 => Some('3'),
            4 => Some('4'),
            5 => Some('5'),
            6 => Some('6'),
            7 => Some('7'),
            8 => Some('8'),
            9 => Some('9'),
            10 => Some('a'),
            11 => Some('b'),
            12 => Some('c'),
            13 => Some('d'),
            14 => Some('e'),
            15 => Some('f'),
            16 => Some('g'),
            17 => Some('h'),
            18 => Some('i'),
            19 => Some('j'),
            20 => Some('k'),
            21 => Some('l'),
            22 => Some('m'),
            23 => Some('n'),
            24 => Some('o'),
            25 => Some('p'),
            26 => Some('q'),
            27 => Some('r'),
            28 => Some('s'),
            29 => Some('t'),
            30 => Some('u'),
            31 => Some('v'),
            32 => Some('w'),
            33 => Some('x'),
            34 => Some('y'),
            35 => Some('z'),
            _ => None,
        };
        digit.ok_or(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn zero_test() {
        let x = super::NumString::new("0", 2);
        assert_eq!(x.is_valid(), true);
        assert_eq!(x.convert(2), Ok("0".to_string()));
        assert_eq!(x.convert(10), Ok("0".to_string()));
    }
}
