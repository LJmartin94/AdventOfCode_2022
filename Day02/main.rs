fn main()
{
	let file_name = "./test_input.txt";
	let input = std::fs::read_to_string(file_name).expect("Wasn't able to read input");
	let str_input_vector: Vec <&str> = input.lines().collect();

	let symbol_rock = 'X'; //1
	let symbol_paper = 'Y'; //2
	let symbol_scissors = 'Z'; //3

	let op_rock = 'A';
	let op_paper = 'B';
	let op_scissors = 'C';

	let mut win_vec = Vec::new();
	win_vec.push(op_rock.to_string() + " " + &symbol_paper.to_string());
	win_vec.push(op_paper.to_string() + " " + &symbol_scissors.to_string());
	win_vec.push(op_scissors.to_string() + " " + &symbol_rock.to_string());

	let mut draw_vec = Vec::new();
	draw_vec.push(op_rock.to_string() + " " + &symbol_rock.to_string());
	draw_vec.push(op_paper.to_string() + " " + &symbol_paper.to_string());
	draw_vec.push(op_scissors.to_string() + " " + &symbol_scissors.to_string());

	let mut loss_vec = Vec::new();
	loss_vec.push(op_rock.to_string() + " " + &symbol_scissors.to_string());
	loss_vec.push(op_paper.to_string() + " " + &symbol_rock.to_string());
	loss_vec.push(op_scissors.to_string() + " " + &symbol_paper.to_string());

	let mut total_score = 0;
	for s in &str_input_vector
	{
		if s.chars().last().unwrap() == symbol_rock{
			total_score += 1;
		}
		else if s.chars().last().unwrap() == symbol_paper{
			total_score += 2;
		}
		else if s.chars().last().unwrap() == symbol_scissors{
			total_score += 3;
		}

		if win_vec.iter().any(|i| i == s){
			total_score += 6;
		}
		else if draw_vec.iter().any(|i| i == s){
			total_score += 3;
		}
		else if loss_vec.iter().any(|i| i == s){
			total_score += 0;
		}
	}
	println!("Total score (first puzzle): {}\n", total_score);

	let symbol_loss = 'X'; // 0
	let symbol_draw = 'Y'; // 3
	let symbol_win = 'Z'; //6

	let mut rock_vec = Vec::new();
	rock_vec.push(op_rock.to_string() + " " + &symbol_draw.to_string());
	rock_vec.push(op_paper.to_string() + " " + &symbol_loss.to_string());
	rock_vec.push(op_scissors.to_string() + " " + &symbol_win.to_string());

	let mut paper_vec = Vec::new();
	paper_vec.push(op_rock.to_string() + " " + &symbol_win.to_string());
	paper_vec.push(op_paper.to_string() + " " + &symbol_draw.to_string());
	paper_vec.push(op_scissors.to_string() + " " + &symbol_loss.to_string());

	let mut scissor_vec = Vec::new();
	scissor_vec.push(op_rock.to_string() + " " + &symbol_loss.to_string());
	scissor_vec.push(op_paper.to_string() + " " + &symbol_win.to_string());
	scissor_vec.push(op_scissors.to_string() + " " + &symbol_draw.to_string());

	total_score = 0;
	for s in &str_input_vector
	{
		if s.chars().last().unwrap() == symbol_loss{
			total_score += 0;
		}
		else if s.chars().last().unwrap() == symbol_draw{
			total_score += 3;
		}
		else if s.chars().last().unwrap() == symbol_win{
			total_score += 6;
		}

		if rock_vec.iter().any(|i| i == s){
			total_score += 1;
		}
		else if paper_vec.iter().any(|i| i == s){
			total_score += 2;
		}
		else if scissor_vec.iter().any(|i| i == s){
			total_score += 3;
		}
	}

	println!("Total score (second puzzle): {}\n", total_score);


}