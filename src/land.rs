use super::dice;

pub struct Land{
    sections: [[[char; 11]; 11]; 4],
}

impl Land{
    pub fn new(aspect: i8) -> Land{
        let mut s = [[['0'; 11]; 11]; 4];
        for i in 0..4{
            s[i] = section_gen((i as i8 + 1 as i8), aspect);
        }
        return Land{sections: s,}
    }
    
    pub fn get_section(&self, n: i8) -> [[char; 11]; 11]{
        return self.sections[(n-1) as usize];
    }
}

pub fn section_gen(sect: i8, aspect: i8)->[[char; 11]; 11]{
    let mut section: [[char; 11]; 11] = [['0'; 11]; 11]; 
    let mut r = 0;
    for i in 0..11{
        for j in 0..11{
            r = dice::roll(2,4);
            if r == 3 || r == 5 || r == 7 {
                section[i][j] = 'E';
            }else if r == 2 {
                section[i][j] = 'D';
            }else if r == 8 {
                section[i][j] = 'V';
            }else if r == 4 {
                section[i][j] = 'C';
            }else if r == 6 {
                section[i][j] = 'R';
            }
        }
    }
    if sect == 1 {
        section[5][5] = 'H';
    }
    return section;
}
