use std::fmt;

pub struct Board {
    fields: [[i32; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let fields = [[0; 8]; 8];
        Board { fields }
    }

    pub fn init_figures(&mut self) {
        let black = [2, 3, 4, 5, 6, 4, 3, 2];
        let white = [2, 3, 4, 6, 5, 4, 3, 2];

        self.fields[1] = [1; 8];
        self.fields[0] = black;

        self.fields[6] = [1; 8];
        self.fields[7] = white;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::from("");
        for row in self.fields {
            string = string + &row.map(|v| v.to_string()).join("|") + &String::from("\n");
        }
        write!(f, "{}", string)
    }
}
