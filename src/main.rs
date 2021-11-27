
fn check(x:Vec<i32>,num:i32)->bool{
	for i in x{
		if i+num==100{
			return false;
		}
	}
	true
}

fn main() {
		let mut cards:Vec<i32> = vec![];
		for i in 10..100{
			if check(cards.clone(),i){
			cards.push(i);
			}
		}
		
		println!("{}",cards.len())
}
