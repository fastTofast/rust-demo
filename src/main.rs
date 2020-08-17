use std::io::stdin;
fn main() {
    println!("Hello, world!");
    let mut instr = String::new();
    loop {
        stdin().read_line(&mut instr).expect("cus error");
        println!("你输入了:{}", instr);
        instr.drain(..);
    }
}
