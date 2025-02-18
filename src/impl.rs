mod secg256;

use: secg256:Secgen256;
fn main() {
    let mut rng = Secgen256::new();
    let randomstuff = rng.genstream(1024);
    println!("data generated: {}", randomstuff.len());
}