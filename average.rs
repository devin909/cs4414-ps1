use std::{os, uint};


fn main() {
  let args: ~[~str] = os::args();
  //println (fmt!("%u", args.len()));
  let mut sum =0 as float;
  for uint::range(1, args.len()) |i| {
    sum+=args[i] as float;
  }
  println(fmt!("%f",sum));

}