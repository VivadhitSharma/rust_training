pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32{
    x-y
}

pub fn mul(x:i32,y:i32)->i32{
    x*y
}

pub fn div(x: f32, y: f32) -> Result<f32, String> {
    if y == 0f32 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}