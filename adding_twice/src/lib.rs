pub fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32{
   move |x| f(f(x))
}

pub fn add_curry(a:i32)->impl Fn(i32) -> i32{
   move |x| x+a 
}