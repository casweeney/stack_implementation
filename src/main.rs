fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    println!("The poped value is {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut user_input = String::new();
    std::io::stdin()
    .read_line(&mut user_input)
    .expect("failed to read inout");

    let user_input: u32 = user_input.trim().parse().expect("Invalid input");

    user_input
}
 
fn main() {
    println!("Let us first create a stack for our use");
    println!("Please mention the size of the stack");

    let stack_size = input();

    let mut stack  = new_stack(stack_size as usize);

    loop{
        println!("\n\n ****** Menu ****** \n");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
        println!("\n Enter your choice: ");

        let choice = input();

        match choice {
            1 => {
                println!("Enter the value to be inserted: ");
                let item = input();
                push(&mut stack, item, stack_size as usize);
            }

            2 => {
                println!("The element which is poped is {:?}", pop(&mut stack));
            }

            3 => println!("The elements are {:?}", stack),

            4 => println!("The size of the stack is {}", size(&stack)),

            5 => break, //println!("\n Exiting"),

            _ => println!("Wrong selection try again"),
        }

        println!("Do you want to continue 1 = Yes / 0 = No");

        let status = input();

        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}