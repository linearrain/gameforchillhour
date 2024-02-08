struct Matrix { // Matrix 3x3 for a game
    rows: [[char; 3]; 3],
    is_draw: bool,
}

impl Matrix { // a function for drawing a matrix
    fn draw_matrix(&self) {
        for i in 0..3 {
            println!("===== ===== =====");
            print!("= {} = = {} = = {} =\n", self.rows[i][0], self.rows[i][1], self.rows[i][2]);
        }
        println!("===== ===== =====");
    }
}

struct Player {
    symbol: char,
    is_winner: bool,
    was_last: bool,
}

impl Player {
    fn choose_place(&self, matrix : &mut Matrix) {
        let mut final_pos : usize = 0;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Error of reading");
            let input : usize = input.trim().parse().expect("Converting error");
            if matrix.rows[(input - 1) / 3][(input - 1) % 3] != 'X' && matrix.rows[(input - 1) / 3][(input - 1) % 3] != '0' { // If in this pos
                final_pos = input;                                                                                            // is a value
                break;                                                                                                        // do not write and repeat the loop
            }

            println!("This column was already used. Try again another one.");
        }

        matrix.rows[(final_pos - 1) / 3][(final_pos - 1) % 3] = self.symbol;
    }
}

fn main() {
    let mut main_matrix = Matrix {
        rows: [[' '; 3]; 3],
        is_draw: false,
    };
    let mut p1 : Player = Player { // The first player, which will play as X
        symbol: 'X',
        is_winner: false,
        was_last: false,
    };
    let mut p2 : Player = Player { // The second one, which will go as 0
        symbol: '0',
        is_winner: false,
        was_last: true,            // Making it true, so when decide function runs, the p1 (X) will be first
    };

    let mut num_itr : usize = 0;

    while num_itr < 9 { // Creating a start matrix by filling it with the numbers from 1 to 9
        main_matrix.rows[num_itr / 3][num_itr % 3] = (num_itr + 1).to_string()
                                                                  .parse().expect("Converting error");
        num_itr += 1;
    }
    println!("Welcome! The game for hour of relax is sincerely glad to see you! In the console you just have to write the number, which is a position, where the X or 0 would be put. The X will start firstable:\n");
    main_matrix.draw_matrix(); // Drawing a start matrix

    decide_turn(&mut p1, &mut p2, &mut main_matrix);
    std::thread::sleep(std::time::Duration::from_secs(10));    // In case user plays in MS Console / Powershell, it is important to keep the console open for a few secs more
}

fn decide_turn(p1 : &mut Player, p2 : &mut Player, matrix : &mut Matrix) {
    while !p1.is_winner && !p2.is_winner && !matrix.is_draw { // In case, there is a win or a draw, this loop should not continue anymore longer
        if p1.was_last {
            p1.was_last = false;
            p2.was_last = true;
            p2.choose_place(matrix);   // Giving a player to choose, where to put 0
        }
        else {
            p2.was_last = false;
            p1.was_last = true;
            p1.choose_place(matrix);   // Giving a player to choose, where to put X
        }
        clearscreen::clear().expect("Console clearance error"); // Clearing the console everytime to make the matrix not move
        matrix.draw_matrix();

        check_winners(p1, matrix);      // Checking for a win of a first player and printing if yes
        check_winners(p2, matrix);      // Checking for a win of a second player and printing if yes
        check_noone(p1, p2, matrix);           //  Checking if there was a draw
    }
}

fn check_winners(player : &mut Player, matrix : &Matrix) {  // A function for checking if player won
                                                            // Checking the rows and columns to see if the current player won
    for itr in 0..3 {
        if (matrix.rows[itr][0] == matrix.rows[itr][1] && matrix.rows[itr][0] == matrix.rows[itr][2] && matrix.rows[itr][0] == player.symbol) || 
           (matrix.rows[0][itr] == matrix.rows[1][itr] && matrix.rows[0][itr] == matrix.rows[2][itr] && matrix.rows[0][itr] == player.symbol) {
            print_winner(player);
            break;
        }
    }
    // Checking the crossings to see if the current player won
    if ((matrix.rows[0][0] == matrix.rows[1][1] && matrix.rows[0][0] == matrix.rows[2][2]) || 
       (matrix.rows[2][0] == matrix.rows[1][1] && matrix.rows[2][0] == matrix.rows[0][2])) && matrix.rows[1][1] == player.symbol {
        print_winner(player);
    }
}

fn print_winner(player : &mut Player) {
    println!("{} is a definite winner right now.", player.symbol);
    player.is_winner = true;                                        // This will stop the loop in decide function
}

fn check_noone(p1 : &mut Player, p2 : &mut Player, matrix : &mut Matrix) {  // The function declared to reduce the amount of checks for non-empty values from 2 to 1 in every iteration
    let mut filled_cells : u8 = 0;                                          // A starting variable for checking how many values got non-empty values
    for i in 0..3 {
        for j in 0..3 {
            if matrix.rows[i][j] == 'X' || matrix.rows[i][j] == '0' {
                filled_cells += 1;
            }
        }
    }
    if filled_cells == 9 && p1.is_winner == false && p2.is_winner == false { // In case everything is non-empty, but there is no winner, we get the draw in the current game
        println!("Nobody wins, there's a draw.");
        matrix.is_draw = true;
    }
}