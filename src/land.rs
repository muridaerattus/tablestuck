use super::dice;

pub fn section_gen(sect: i8, aspect: i8)->[[char; 11]; 11]{
    let mut section: [[char; 11]; 11] = [['0'; 11]; 11]; 
    let mut r = 0;
    for i in 0..11{
        for j in 0..11{
            r = dice::roll(2,4);
            if(r == 3 || r == 5 || r == 7){
                section[i][j] = 'E';
            }else if(r == 2){
                section[i][j] = 'D';
            }else if(r == 8){
                section[i][j] = 'V';
            }else if(r == 4){
                section[i][j] = 'C';
            }else if(r == 6){
                section[i][j] = 'R';
            }
        }
    }
    section[5][5] = 'H';
    return section;
}
