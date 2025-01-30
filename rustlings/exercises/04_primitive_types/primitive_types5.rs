fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;

    println!("{name} is {age} years old");

    // Notes by Carlo: I can also use it to swap. This is very Javascripty, but it works.
    let a = "Carlo";
    let b = "Awesome";

    println!("a: {a} | b: {b}");

    let (b, a) = (a, b);

    println!("a: {a} | b: {b}");
}
