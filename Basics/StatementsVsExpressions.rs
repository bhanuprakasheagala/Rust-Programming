/*
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.

Function definitions are also statements;
*/
fn main(){
    //let a = 6; //Statement: does not return anything

    /*Statements do not return values. Therefore, you can’t assign a let statement to another variable,
    as the following code tries to do; you’ll get an error: */
    //let x = (let y = 6);

    /*
    Calling a function is an expression. Calling a macro is an expression.
    A new scope block created with curly brackets is an expression, for example:
    */

    let y = {
        let x = 3;
        x + 1  
        /* x + 1 line doesn’t have a semicolon at the end, which is unlike most of
        the lines you’ve seen so far. Expressions do not include ending semicolons.
        If you add a semicolon to the end of an expression,
        you turn it into a statement, and it will then not return a value.
        */
    };

    println!("The value of y is: {y}");
}