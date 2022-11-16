fn main() {
    let num1 = 4;
    println!("{}",num1);
    
    println!("{}", "Hello, world!");
    
    let mut num2 = 2;
    num2 = 12;
    println!("{}", num2);
    
    println!("{}",num1 * num2);

    let mut x = 10;
    let y = 6;
    println!("The value of x = {} and y = {}",x,y);
    x = x - y;
    println!("The result value is {}",x);
    
    println!("{}",sub2(x,y));


 ///// integer //////
    let x:u8 = 255; // you can't use u8 for the value greater than 255 or less than 0
    let z:i16 = 10;
    let y:f32 = 10.0;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);


///// array ///

    let mut arr = [1,2,3];
    arr[0] = 123;
    println!("{}", arr[0]);


//// tupple ////

    let mut tup = ("Muhammad",68,8.6,true);
    tup.0 = "Mirza Baig";
    println!("{}", tup.0);


/// if else ///
    let x = 10;
    
    if x==11{
        println!("Yes x is equal to {}",x);
    }else{
        println!("No, x is not equal to {}",x);
    }


/// loop ///

    let mut count = 0;
    
    loop{
        println!("without adding {} ",count);
        count = count + 1;
        println!("with adding {}",count);
        if count == 10{
            break;
        }
    }

}

fn sub1() {
    let a = 6;
    let b = 4;
    let c = a * b;
    println!("{}", c);
}

fn sub2(m:i32,u:i32)->i32{
    m*u // if you don't want to use return keyword, use will not have to use colon ";", otherwise syntax will be like this "return m*y;"
}