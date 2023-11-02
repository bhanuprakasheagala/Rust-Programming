/*
Returning Values from the loop
One of the uses of a loop is to retry an operation you know might fail, such as checking 
whether a thread has completed its job. You might also need to pass the result of that operation
out of the loop to the rest of your code. To do this, you can add the value you want returned
after the break expression you use to stop the loop; that value will be returned out of the 
loop so you can use it.
*/
fn main(){
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };

    println!("The result is {result}");
}