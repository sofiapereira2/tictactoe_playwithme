fn main(){
    let mut board =[
        ['_', '_', '_'],
        ['_', '_', '_'],
        ['_', '_', '_']
    ];
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", board[i][j]);
        }
        println!();
    }
 
    loop {
        println!("Enter the row number (0-2): ");
        let mut row = String::new();
        std::io::stdin().read_line(&mut row).unwrap();
        let row: usize = row.trim().parse().unwrap();
 
        println!("Enter the column number (0-2): ");
        let mut col = String::new();
        std::io::stdin().read_line(&mut col).unwrap();
        let col: usize = col.trim().parse().unwrap();
        
        if row > 2 || col > 2 {
            println!("Invalid position");
            continue;
        }
 
        if board[row][col] != '_' {
            println!("Position already taken");
            continue;
        }
  
        println!("Enter the value (X or O): ");
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        let value: char = value.trim().parse().unwrap();
 
        if value != 'X' && value != 'O' {
            println!("Invalid value");
            continue;
        }
 
        board[row][col] = value;
 
        let mut draw = true;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == '_' {
                    draw = false;
                    break;
                }
            }
        }
 
        let mut x_wins = false;
        let mut o_wins = false;
 
        for i in 0..3 {
            if board[i][0] == 'X' && board[i][1] == 'X' && board[i][2] == 'X' {
                x_wins = true;
                break;
            }
            if board[i][0] == 'O' && board[i][1] == 'O' && board[i][2] == 'O' {
                o_wins = true;
                break;
            }
        }
 
        for i in 0..3 {
            if board[0][i] == 'X' && board[1][i] == 'X' && board[2][i] == 'X' {
                x_wins = true;
                break;
            }
            if board[0][i] == 'O' && board[1][i] == 'O' && board[2][i] == 'O' {
                o_wins = true;
                break;
            }
        }
 
        if board[0][0] == 'X' && board[1][1] == 'X' && board[2][2] == 'X' {
            x_wins = true;
        }
 
        if board[0][0] == 'O' && board[1][1] == 'O' && board[2][2] == 'O' {
            o_wins = true;
        }
 
        if board[0][2] == 'X' && board[1][1] == 'X' && board[2][0] == 'X' {
            x_wins = true;
        }
 
        if board[0][2] == 'O' && board[1][1] == 'O' && board[2][0] == 'O' {
            o_wins = true;
        }
 
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", board[i][j]);
            }
            println!();
        }
 
        
        if x_wins {
            println!("X wins!");
            break;
        }else if o_wins {
            println!("O wins!");
            break;
        }else{
            if draw {
                println!("Draw!");
                break;
            }
        }
 
    }
}