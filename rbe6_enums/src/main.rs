#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// A function which takes an enum as an argument
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

//type aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

//Implement methods to enum
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// use implementation
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// Testcase: linked-list
use crate::List::*;

enum List {
    //Cons: tuple that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: node that signifies the end of the linked list
    End,
}

impl List {
    // consume a list and return the same list with a new element on first item
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of a list, recursion
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well
        match *self {
            // fi there is at least one element in the list
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: an emply list has zero length
            End => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            End => {
                format!("End")
            }
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // For aliases
    let additions = Operations::Add;
    let soustract = Operations::Substract;
    println!("Additions: {}", additions.run(2, 2));
    println!("Soustractions: {}", soustract.run(6, 4));

    //use
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    //Automatically use each name inside Work
    use crate::Work::*;

    //Equivalent to Status::Poor
    let status = Poor;
    //Equivalent to Work::Civilian
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Sorliers fight!"),
    };

    //Test case the list
    // Create an empty linked list
    let mut list = List::End;

    println!("{}", list.stringify());

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify())

}
