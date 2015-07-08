use std::thread;

struct Philospher {
    name: String,
}

impl Philospher {
    fn new(name: &str) -> Philospher {
        Philospher {
            name: name.to_string(),
        }
}

    fn eat(&self) {
        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
        }
    }


fn main() {

let philosphers = vec![
        Philospher::new("Baruch Spinoza"),
        Philospher::new("Gilles Deleuze"),
        Philospher::new("Karl Marx"),
        Philospher::new("Friedrich Nietzsche"),
        Philospher::new("Michel Foucault"),
        ];

        for p in &philosphers {
            p.eat();
        }
}
