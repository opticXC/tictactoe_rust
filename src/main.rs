

#[macro_use]
extern crate text_io;

 
static mut GRID:Vec<String> = Vec::new();
static mut GRID_C:Vec<i32> = Vec::new() ;


static mut TURN: i32 = 1;

fn main(){
    grid_set(); //initialise grids
    gridup();   //print grid to terminal
    unsafe{

        while TURN <=9 {  //main loop
            update();
            TURN +=1;
        }
        println!("Its a Draw");
    }
}

fn update(){ //runs function in order everytime ~~probably could have done this in main loop instead of creating a function ~~
    
    get_up();
    gridup();
    chech_win();
}

fn gridup(){ //prints grid to std::out 
    let mut line1:String = String::new();
    let mut line2:String = String::new();
    let mut line3:String = String::new();

    let breaker: String = "-------".to_string();
    
    unsafe {  //appending grid values to strings CUZ NO MULTIPLE AREGUMENTS IN PRINT STATEMENT??????? => i didnt know u could do it when i created this . mb
        for i in [0,1,2,3,4,5,6,7,8] {
            if i <=2{
                line1.push_str(&GRID[i].to_string() );
                line1.push_str("|");
            }else if i <=5 {
                line2.push_str(&GRID[i].to_string() );
                line2.push_str("|");
            }else if i<=8{
                line3.push_str(&GRID[i].to_string() );
                line3.push_str("|");
            }
        }
    }

    

    println!("{} ", &line1.to_string());
    println!("{} ", &breaker.to_string());
    println!("{} ", &line2.to_string());
    println!("{} ", &breaker.to_string());
    println!("{} ", &line3.to_string());


}


fn get_up(){  //takes user input to replace 1 grid value (RECURSIVE)
    unsafe { if TURN %2 !=0{println!("player 1(O) 's Turn")}else {
        println!("player 2(X)'s Turn");

    } }
    println!("enter a number in the valid range");
    let inp:i32 = read!();
    
    
    let input:usize =(inp -1).try_into().unwrap();

    


    if [0,1,2,3,4,5,6,7,8].contains(&input) == false  {
        
        println!("please enter a number in the valid range");
        get_up();

    }else{
        

        unsafe {if GRID_C[input] == 0{
        
            if TURN %2 ==0{
                GRID[input] = "X".to_string();
        
            }else{
                GRID[input] = "O".to_string();
            }
            GRID_C[input] = 1;
        
        }else if GRID_C[input] ==1 {
            println!("THE ENTERNED CELL HAS ALREADY BEEN OCCUPIED");
            get_up();
        }
    }
        
    }
    

}




fn grid_set(){ //1st function to run ( sets up the initial vector grids)
    let mut i:i32 = 1;
    while i <=9{
        unsafe{
            
            GRID.push(i.to_string());
            GRID_C.push(0);
            i +=1;
        }
    }

}

fn exit(){ //exit program after printing winner
    unsafe{
    if TURN %2 ==0 {

        println!("player 1 (X) has won " );
        std::process::exit(0);
    }else {
        println!("player 2 (O) has won " );
        std::process::exit(0);
    }
}
}


fn chech_win(){
    unsafe {
    //horizontal line checks
    if GRID[0] == GRID[1] && GRID[1] == GRID[2]{
        exit();
    }else if GRID[3] == GRID[4] && GRID[4] == GRID[5] {
        exit();
    }else if GRID[6] == GRID[7] && GRID[7] == GRID[8] {
        exit();

    //vertical line checks
    }else if GRID[0] == GRID[3] && GRID[3] == GRID[6] {
        exit();
    }else if GRID[1] == GRID[4] && GRID[4] == GRID[7] {
        exit()
    }else if GRID[2] == GRID[5] && GRID[5] == GRID[8] {
        exit();
    

    //diagonal line checks
    }else if GRID[0] == GRID[4]&& GRID[4] == GRID[8] {
        exit();
    }else if GRID[2] == GRID[4] && GRID[4] == GRID[6]{
        exit();
    }
}
}