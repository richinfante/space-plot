
extern crate space_plot;

fn main() {
  println!("Blank example\n{}", space_plot::render_point(vec![space_plot::Point {
    x: 0.0,
    y: 0.0,
    name: None,
    color: None,
    point: None
  }, space_plot::Point {
    x: -70.0,
    y: 40.0,
    name: None,
    color: None,
    point: None,
  }], space_plot::Plot::make_blank(40, 30)));

  println!("Map example:\n{}", space_plot::render_point(vec![space_plot::Point {
    x: 0.0,
    y: 0.0,
    name: None,
    color: None,
    point: None
  }, space_plot::Point {
    x: -70.0,
    y: 40.0,
    name: None,
    color: None,
    point: None,
  }], space_plot::Plot::default()));
}