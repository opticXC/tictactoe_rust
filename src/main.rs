

#[macro_use]
extern crate text_io;

 
static mut GRID:Vec<String> = Vec::new();
static mut GRID_C:Vec<i32> = Vec::new() ;


static mut TURN: i32 = 1;

fn main(){
    grid_set();
    gridup();
    unsafe{

        while TURN <=9 {
            
            TURN +=1;
            update();
        }
    }
}

fn update(){
    
    get_up();
    gridup();
    chech_win();
}

fn gridup(){
    let mut line1:String = String::new();
    let mut line2:String = String::new();
    let mut line3:String = String::new();

    let breaker: String = "---------".to_string();

    unsafe {
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

    unsafe {println!("\n Turn number : {} \n", TURN );}

    println!("{} ", &line1.to_string());
    println!("{} ", &breaker.to_string());
    println!("{} ", &line2.to_string());
    println!("{} ", &breaker.to_string());
    println!("{} ", &line3.to_string());


}


fn get_up(){
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




fn grid_set(){
    let mut i:i32 = 1;
    while i <=9{
        unsafe{
            
            GRID.push(i.to_string());
            GRID_C.push(0);
            i +=1;
        }
    }

}

fn chech_win(){
    
}