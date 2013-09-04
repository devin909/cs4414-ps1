use std::{os, uint, float};


fn main() {
  let args: ~[~str] = os::args();
  //println (fmt!("%u", args.len()));
  let mut sum =0.0f;
  let mut count = 0.0f;
  for uint::range(1, args.len()) |i| {
  	match float::from_str(args[i]) {
      Some(num) => {
          sum += num;
          count+=1.0;
          //println(fmt!("%f %f", num, sum));
      }
      None => {
          println(fmt!("Bad input: %s", args[i]))
      }
    }
  }
  sum /= count;
  println(fmt!("Average: %f",sum));

}