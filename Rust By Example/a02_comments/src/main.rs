// This is an example of a line comment 
// There are two slashes at the beginning of the line
// And nothing written inside these will be read by the compiler


/*
* This is another type of comment, a block comment, In general,
* line comment are the recommended comment style . But 
* block comment are extremely useful for temporarily disabling chunks of code. /* Block comments 
can be /* nested,  */ */ so it takes only a few keystrokes to comment out everything in this main() function.
*/
fn main(){
    let x = 5 +/*90 + */ 5;
    println!("is 'x' 10 or 100?  x = {}", x);
}