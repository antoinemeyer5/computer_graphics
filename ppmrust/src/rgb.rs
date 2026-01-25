/****************************************************************** STRUCTURE */

// Clone: allows explicit copying of a type’s value.
// - This method is responsible for creating a new copy of the value.
// Copy: signifies that a type can be copied bit by bit.
// - It simply marks a type as being eligible for bitwise copying.

#[derive(Clone, Copy)]
pub struct Rgb {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}
