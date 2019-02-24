// Structure contains a single i32
#[derive(Debug)]
struct Structure(i32);

// Double structure makes it printable !? 
//#[derive(Debug)]
//struct Deep(Structure);
/*
 * format!("Hello");                 // => "Hello"
 * format!("Hello, {}!", "world");   // => "Hello, world!"
 * format!("The number is {}", 1);   // => "The number is 1"
 * format!("{:?}", (3, 4));          // => "(3, 4)"
 * format!("{value}", value=4);      // => "4"
 * format!("{} {}", 1, 2);           // => "1 2"
 * format!("{:04}", 42);             // => "0042" with leading zeros
 * format!("{argument}", argument = "test");   // => "test"
 * format!("{name} {}", 1, name = 2);          // => "2 1"
 * format!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b" 
 * nothing =>Display
 * ?  => Debug
 * x? => Debug with lower-case hexadecimal integers
 * X? => Debug with upper-case hexadecimal integers
 * o  => Octal
 * x  => LowerHex
 * X  => UpperHex
 * p  => Pointer
 * b  => Binary
 * e  => LowerExp
 * E  => UpperExp
 * format!("{1} {} {0} {}", 1, 2); // => "2 1 1 2"
 * fmt::Display implementations assert that the type can be faithfully represented as a 
 * UTF-8 string at all times. It is not expected that all types implement the Display 
 * trait.
 * fmt::Debug implementations should be implemented for all public types. Output will 
 * typically represent the internal state as faithfully as possible. The purpose of the 
 * Debug trait is to facilitate debugging Rust code. In most cases, using 
 * #[derive(Debug)] 
 * is sufficient and recommended. 
 */
fn main()
{
 println!("{1:?} {0:?} is the {actor:?} name","Slater", "Cristrain", actor="actor's");
 println!("Now {:?} will print!",Structure(3));
// println!("Now {:?} will print!",Deep(Structure(7)));
}

