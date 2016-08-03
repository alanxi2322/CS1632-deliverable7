// CS1632 - Deliverable7 
// Lun Xi 

use std::io;
extern crate rand;
use rand::Rng;


//Declaring structure to keep track of all the data
struct Points {
    win: i32,
	tie: i32,
    lose: i32,
	rock: i32,
	paper: i32,
    scissor: i32,
}

// Function for printing the input line
fn input_prompt() {
    println!("Enter Choice (r, p, s) or q to quit") ;
}

// Function that'll print the final stats at the end of the 
// program when the user press q to quit

fn final_stats(win: i32, tie: i32 , lose: i32, rock: i32, paper: i32 , scissor: i32, total: i32)
{
	let t :f32;
	t = total as f32;
	let mut w : f32 = win as f32;
	let mut tt : f32 = tie as f32;
	let mut l : f32 = lose as f32;
	
	
	// Calculating the percentage of wins, tie and losses
	if total != 0 
	{
		// Calculating percentage of winning
		w = w/t *100.0;
		//Calculating percentage of ties
		tt = tt/t *100.0;
		//Calculating percentage of losses
		l = l/t * 100.0;
	}
	// Printing all the information
	
	println!("Wins : {} ({:.2}%)", win, w);
	println!("Ties : {} ({:.2}%)", tie, tt);
	println!("Losses : {} ({:.2}%)", lose, l);
	
	println!("Rock : {}", rock);
	println!("Paper : {}", paper);
	println!("Scissor : {} ", scissor);
	
}

// Function that adds one to the variable passes to it by reference
// Value is pass as a reference to this function and then one is added to it
fn add_one(e: &mut i32){
    *e+= 1;
}


fn inspect(t: Total) {
    match t {
        Total::Total(i) => println!("Total : {}.", i),
    }
}

//Declaring Enum Having an integer name Total
enum Total {
    Total(i32),    
}


// Main function

fn main() {
	
	//Making an instance of class Points and initializing all its members by 0
	let mut point = Points { win: 0, tie: 0, lose: 0, rock: 0, paper: 0, scissor: 0 };

	// Declaring a boolean
	let mut done = false;
	
	// While user do not enter "q" the loop will continue
	while !done {
	
		//Declaring String
		let mut guess = String::new();

		//Calling input prompt function 
		input_prompt();
		
		//Taking input from the user
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
		
		//Random function which gets the number from 0 to 2 randomly
		let opponent = rand::thread_rng().gen_range(0, 3);
		
		let mut player = 0;
		
		//Checking whether user has entered q or not
		if guess.trim() == "q"{
		
			// Calculating Total games by summing up all the turns
			let total = Total::Total(point.rock+ point.paper + point.scissor);
			println!("Program Stats");
			inspect(total);
			
			let t = point.rock+ point.paper + point.scissor;
			
			//Calling function and passing it all the arguments so that it may print the final results
			final_stats(point.win, point.tie, point.lose, point.rock, point.paper, point.scissor, t);
			
			//Done = true means the game is finished now
			done = true;
		}
		else
		{
			//If user choose Rock
			if guess.trim() == "r"{
				println!("Player chose : Rock");
				
				//Assigning 0 if user chooses Rock
				player = 0;
				
				//+1 will be added to rock variable
				add_one(&mut point.rock);
			}
			//If user chooses paper
			else if guess.trim() == "p"{
				println!("Player chose : Paper");
				
				//Assigning 1 if user chooses Paper
				player = 1;
				
				//+1 will be added to Paper variable
				add_one(&mut point.paper);
			}
			//If user chooses Scissor
			else if guess.trim() == "s"{
				println!("Player chose : Scissor");
				
				//Assigning 2 if user chooses Scissor
				player = 2;
				
				//+1 will be added to Scissor variable
				add_one(&mut point.scissor);
			}
			
			//If random number is 0 that means opponent has selected rock
			if opponent == 0{
				println!("Opponent chose : Rock");
			}
			//If random number is 1 that means opponent has selected Paper
			else if opponent == 1{
				println!("Opponent chose : Paper");
			}
			//If random number is 2 that means opponent has selected Scissor
			else if opponent == 2{
				println!("Opponent chose : Scissor");
			}
			
			//If both player and opponent has same sign the game will draw
			if player == opponent{
				println!("It's a Tie!");
				add_one(&mut point.tie);
			}
			//If User selects rock and opponent selects Scissor then player wins
			else if player == 0 && opponent == 2{
				println!("You Win!");
				add_one(&mut point.win);
			}
			//If User selects rock and opponent selects Paper then player lose
			else if player == 0 && opponent == 1{
				println!("You Lose!");
				add_one(&mut point.lose);
			}
			//If User selects Paper and opponent selects Rock then player wins
			else if player == 1 && opponent == 0{
				println!("You Win!");
				add_one(&mut point.win);
			}
			//If User selects Paper and opponent selects Scissor then player Lose
			else if player == 1 && opponent == 2{
				println!("You Lose!");
				add_one(&mut point.lose);
			}
			//If User selects Scissor and opponent selects Rock then player Lose
			else if player == 2 && opponent == 0{
				println!("You Lose!");
				add_one(&mut point.lose);
			}
			//If User selects Scissor and opponent selects Paper then player wins
			else if player == 2 && opponent == 1{
				println!("You Win!");
				add_one(&mut point.win);
			}
		}
	}
}