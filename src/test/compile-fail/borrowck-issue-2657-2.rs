fn main() {
let x = Some(~1);
match x {
  Some(ref y) => {
    let _b = move *y; //~ ERROR moving out of dereference of immutable & pointer
  }
  _ => {}
}
}
