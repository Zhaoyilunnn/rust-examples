// Some summary
//   If variable is allocated on stack, by default <copy>
//   If allocated on heap, by default <move>

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn box_test() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn drop_test() {
    let x = ToDrop;
    println!("Made a ToDrop");
}

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

// This function will not take ownership
fn not_destroy_box(c: &Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn ownership_and_moves_test() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);
    // But this function will not take the ownership
    //not_destroy_box(&b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
    // if you use not_destroy_box, then you can still print b
}

fn mutability_test() {
    let immutable_box = Box::new(1.5);

    println!("Immutable box contains {}", immutable_box);

    let mut mutable_box = immutable_box;

    println!("Mutable box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 2.2;
    println!("Mutable box contains {}", mutable_box);
}

// The parent variable cannot be used after partial move
// but the parts that are only referenced (and not moved) can still be used
fn partial_move_test() {
    let a: i32 = 1;
    // a is allocated on heap, and this will not move a but copy a
    let b = a;

    println!("a is {a}");
    println!("b is {b}");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<i32>,
    }

    let person = Person {
        name: String::from("Bob"),
        age: Box::new(10),
    };

    println!("The person is {:?}", person);
    let Person { name, ref age } = person;

    // Since person.age is not moved, person.age can still be used
    println!("Age of the person is {}", person.age);

    // But person cannot be used
    // println!("The person is {:?}", person);
    // TODO: Try uncommenting above line
}

// failed_borrow
fn failed_borrow<'a>() {
    let _x = 12;
    // let y: &'a i32 = &_x;
    // error[E0597]: `x` does not live long enough
}

// ERROR
// fn compare_two_strings(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

static Z: &str = "hello";

// compiler will not assume the lifetime of the return value
// thus it will not compile without `'static`
fn longer_string_test(x: &str, y: &str) -> &'static str {
    Z
}

fn longer_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn is_longer(x: &str, y: &str) -> bool {
//     x.len() > y.len()
// }

fn return_value(x: &str, y: &str) -> String {
    let z = String::from("hello");
    z
}

// lifetimes
fn test_lifetimes() {
    println!("Test lifetimes");
    let i = 3; // lifetime for `i` starts. It ends at the end of the block
    {
        let r = &i; // `r` lifetime starts. It ends at the end of the block
        println!("r: {}", r);
    } // `r` goes out of scope

    {
        let r = &i; // `r` lifetime starts. It ends at the end of the block
        println!("r: {}", r);
    }

    failed_borrow();

    println!("Test lifetimes compare two strings' length");
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longer_string(s1.as_str(), s2);
    println!("The longer string is {}", result);

    longer_string_test(s1.as_str(), s2);

    return_value(s1.as_str(), s2);
}

fn main() {
    box_test();
    drop_test();
    ownership_and_moves_test();
    mutability_test();
    partial_move_test();
    test_lifetimes()
}
