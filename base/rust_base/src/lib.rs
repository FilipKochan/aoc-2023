pub mod aoc {
    use std::{
        env::args,
        fs::File,
        io::{BufRead, BufReader, Lines},
    };

    pub struct Parser {
        file: BufReader<File>,
    }

    impl Parser {
        pub fn new() -> Self {
            let name = args()
                .nth(1)
                .unwrap_or_else(|| panic!("Input file must be provided."));
            let file = BufReader::new(
                File::open(name).unwrap_or_else(|_| panic!("File could not be opened.")),
            );
            Self { file }
        }

        pub fn parse_by_lines<T>(self, f: fn(String) -> T) -> Vec<T> {
            self.file.lines().flatten().map(f).collect()
        }

        pub fn parse_all<T>(self, f: fn(Lines<BufReader<File>>) -> T) -> T {
            f(self.file.lines())
        }
    }
}
