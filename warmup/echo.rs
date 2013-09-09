use std::{os, uint};

struct Point {
    x: float,
    y: float
}
fn main() {
  let args: ~[~str] = os::args();
  let asdf = 3+ args.len();
  //println (fmt!("%u", args.len()));
  for uint::range(1, args.len()) |i| {
    print (args[i] + " ");
  }
  println("");

 //  let mut mypoint = Point { x: 0.0, y: 1.0 };
	// let origin = Point { x: 0.0, y: 0.0 };

	// mypoint.y += 1.0; // mypoint is mutable, and its fields as well
 //  match mypoint {
 //    Point { x: 0.0, y: yyy } => { println(mypoint.x.to_str());                     }
 //    Point { x: xx,  y: yy } => { println(xx.to_str() + " " + yy.to_str()); }
//}
}