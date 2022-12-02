fn main()
{
	let file_name = "./input";
	println!("File name: {}", file_name);

	let input = std::fs::read_to_string(file_name).expect("Wasn't able to read input");

	let str_input_vector: Vec <&str> = input.lines().collect();
	let mut int_input_vector =  Vec::new();

	let mut sum = 0;
	for s in str_input_vector{
		if s != ""{
			sum += s.parse::<i32>().expect("Couldn't convert string to int");
		}
		else {
			int_input_vector.push(sum);
			sum = 0;
		}
	}
	int_input_vector.push(sum);

	let mut highest = 0;
	for i in int_input_vector{
		if i > highest{
			highest = i;
		}
		println!("{}",i);
	}

	println!("Highest value found was {}", highest);
}