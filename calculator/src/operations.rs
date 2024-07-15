
pub fn add(x: i32, y: i32) -> i32  {
    x + y
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

pub fn div(x: f32, y: f32) -> Result<f32, String> {
    if y == 0f32 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(4, 4), 8);
        assert_eq!(add(-2, 2), 0);
        assert_eq!(add(-0, 0), 0);
        assert_eq!(add(-2, -8), -10);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(sub(-1, -0), -1);
        assert_eq!(sub(25, 50), -25);
        assert_eq!(sub(7, 7), 0);
        assert_eq!(sub(-3, -5), 2);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(mul(0, 0), 0);
        assert_eq!(mul(-3, -9), 27);
        assert_eq!(mul(-0, -10), 0);
        assert_eq!(mul(-0, -0), 0);
        assert_eq!(mul(0, 1), 0);
        assert_eq!(mul(1, 0), 0);
        assert_eq!(mul(1, 1), 1);
        assert_eq!(mul(-2, 5), -10);
    }

    #[test]
    fn test_division() {
        assert_eq!(div(0.0, 7.0), Ok(0.0));
        assert_eq!(div(7.0, 0.0), Err(String::from("Cannot divide by zero")));
        assert_eq!(div(-4.0, -0.0), Err(String::from("Cannot divide by zero")));
        assert_eq!(div(-0.0, -4.0), Ok(0.0));
        assert_eq!(div(-0.0, 4.0), Ok(0.0));
        assert_eq!(div(0.0, -4.0), Ok(0.0));
        assert_eq!(div(2.0, 4.0), Ok(0.5));
        assert_eq!(div(4.0, 2.0), Ok(2.0));
    }
}