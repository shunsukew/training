trait DuckLike {
    fn quack(&self);
    fn walk(&self) {
        println!("walking")
    }
}

struct Duck;
struct Tsuchinoko;

impl DuckLike for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        println!("mew");
    }

    fn walk(&self) {
        println!("wriggling");
    }
}

impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn main() {
    let duck = Duck;
    let tsutinoko = Tsuchinoko;
    let i = 3;
    duck.quack();
    tsutinoko.quack();
    i.quack();
}
