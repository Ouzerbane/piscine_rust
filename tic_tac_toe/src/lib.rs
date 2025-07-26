pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {

for i in ['O', 'X'] {
    if diagonals(i,table){
       return format!("player {} won",i).to_string()
    };
    if horizontal(i,table){
     return format!("player {} won",i).to_string()
    };
    if vertical(i,table){
       return format!("player {} won",i).to_string()
    };
}
"tie".to_string()


}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
     for i in 0..table.len() {
        if i == 0 {
       for x in 0..table[i].len() {
            if table[x][x] != player {
               break ;
            }
            if x == 2 {
                return true ;
            }
        }
    }
       if i == 2 {
       for x in 0..table[i].len() {
            if table[x][i-x] != player {
               break ;
            }
            if x == 2 {
                return true ;
            }
        }
    }
    
    }
    false
    
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
     for i in 0..table.len() {
        for x in 0..table[i].len() {
            if table[i][x] != player {
               break ;
            }
            if x == 2 {
                return true ;
            }
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
     for i in 0..table.len() {
        for x in 0..table[i].len() {
            if table[x][i] != player {
               break ;
            }
            if x == 2 {
                return true ;
            }
        }
    }
    false
}