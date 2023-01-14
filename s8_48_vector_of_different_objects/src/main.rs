trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

// setting function for human and animal
struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("Hello, my name is {}", self.name());  // human will be "Hello, my name is {name}"
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name()); // cat will say ("{name} says meow")
    }
}

enum Creature
{
    Human(Human),
    Cat(Cat)
}

fn via_enums()
{
    let mut creatures = Vec::new();
    // Using an enum:
    creatures.push(Creature::Human(
        Human { name: "John" }  // 
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Fluffy" }
    ));

    for c in creatures
    {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }
}

fn via_animal_box() {
    // This won't work, failing with "doesn't have a size known at
    // compile-time" error.
    // let mut animals:Vec<dyn Animal>= Vec::new();

    let mut animals:Vec<Box<dyn Animal>>= Vec::new();

    animals.push(Box::new(Human{name: "John"})); // name for animals.Human
    animals.push(Box::new(Cat{name: "Fluffy"}));  // name for animals.Cat

    for a in animals
    {
        // A nice thing is that contrary to C++, boxes allows normal syntax for
        // method calls:
        a.talk();
    }
}

fn main() {
    // This approach won't work:
    // let mut creatures = Vec::new();
    // creatures.push(Human{name: "John"});
    // creatures.push(Cat{name: "Fluffy"});

    via_enums();  // calling for the via_enums function
    via_animal_box();  // calling for the via_animal_box function
}
