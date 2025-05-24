pub fn invert_the_case(s: String) -> String {
	
	let mut result = String::new();
	
	for c in s.chars() {
	
		if c.is_lowercase() {
	
			result.push_str(&c.to_uppercase().to_string());
	
		} else if c.is_uppercase() {
	
			result.push_str(&c.to_lowercase().to_string());
	
		} else {
	
			result.push(c);
		}
	}
	
	result
}

fn main() {
	
	let examples = vec!["Hello", "Привет", "goodbye"];
	
	for example in examples {
	
		let input = example.to_string();
		let output = invert_the_case(input.clone());

		println!("{} -> {}", input, output);
		
	}
}

#[cfg(test)]
mod tests {
	
	use super::*;

	#[test]
	fn test() {
	
		let data = [
	
			("Hello", "hELLO"),
			("Привет", "пРИВЕТ"),
	
		];

		for (a, b) in data.iter() {
	
			assert_eq!(invert_the_case(a.to_string()), b.to_string());
			assert_eq!(invert_the_case(b.to_string()), a.to_string());
		}
	}
}
