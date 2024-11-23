fn main() {
    println!("Hello, world!");

    let mut a: i32 =10;
    a=15;
    let b: i32 = 100;
    let c: i32 = 1000;

    println!("The Number is {} {} {}",a,b,c);

    let char: char ='G';
    println!("Character is {}",char);

    let flot_num: f64 = 3.14;

    println!("Float Number is {}",flot_num);

    const xy: i32 = 10;

    println!("Const Value {}",xy);

    let mut my_name: String = String::from("BHOLU");

    let lower_case: String = "Bholu".to_string();

    println!("My Name {} And lower Case is {}",my_name,lower_case);

    let mut My_vec: Vec<u64> = Vec::new();
    My_vec.push(1);
    My_vec.push(2);

    let length_vector: usize = My_vec.len();

    println!("My vector/Array is {:?} and its length is {}",My_vec,length_vector);

    struct Rectangle{
        length: u32,
        width : u32,
    }

   let My_Rectangle: Rectangle=Rectangle{
        length: 10,
        width : 15,
    };

    println!("length and Reactangle is {} and {}",My_Rectangle.length,My_Rectangle.width);

   
    let c=sum(10, 20);
    println!("sum of a+v={}", c);

    let t=sub(20, 10);
    println!("sum of a+v={}", t);
   
    let v=mul(20, 10);
    println!("sum of a+v={}", v);

    let modulo: u32 = modulo(10, 3);
    println!("Modulo of two numbers is: {}", modulo);
}
        fn sum(a : u32, v : u32)-> u32{
         a+v
    }
        fn sub(a : u32, v : u32)-> u32{
        a-v
    }
        fn mul(a : u32, v : u32)-> u32{
        a*v
    }

        fn div(a: u32, b: u32) -> u32 {
         a / b
    }

        fn modulo(a: u32, b: u32) -> u32 {
        a % b 
    }

