fn main() {
    move_sample();
    clone_sample();
}


fn move_sample() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
    //s1 was moved to variable s2
    //println!("{}, world!", s2);
}

fn clone_sample() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
