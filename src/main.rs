//mod greetings;
/*pub mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::primitives::compound;
use how_you_hold_data_for_operations::primitives::scalar;
use how_you_hold_data_for_operations::derived::user_defined;
extern crate hello_world_lib;
///Optionally oad each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
//use greetings::{spanish, french,english};
fn main() {
println!("Hello, world!");
//println!("{}", default_greeting());
/*println!("{}", spanish::default_greeting());
println!("{}", french::default_greeting());
println!("{}", hello_world_lib::greeting_from_lib());
println!("{}", english::default_greeting());*/
how_you_hold_data_for_operations::primitives::compound::array_example();
how_you_hold_data_for_operations::primitives::compound::slice_example();
how_you_hold_data_for_operations::primitives::compound::tuple_example();
how_you_hold_data_for_operations::primitives::scalar::get_integer();
how_you_hold_data_for_operations::primitives::scalar::is_true();
how_you_hold_data_for_operations::primitives::scalar::floating_point_types();
}
//how_you_hold_data_for_operations::primitives::compound;
compound::main();*/
pub mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::primitives::compound;
use how_you_hold_data_for_operations::primitives::scalar;
//use how_you_hold_data_for_operations::derived::user_defined;
extern crate hello_world_lib;

fn main() {
    println!("Hello, world!");
    compound::array_example();
    compound::slice_example();
    compound::tuple_example();
    scalar::get_integer();
    scalar::is_true();
    scalar::floating_point_types();
}

