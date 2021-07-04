use std::f64::consts::PI;
use geo::algorithm::map_coords::MapCoords;
use geo::polygon;
use proj::Proj;
use geo_types::{Polygon,Coordinate};
use tile_grid::Grid;

pub fn transform_polygon(func: Proj, poly: &Polygon<f64>) -> Polygon<f64> {
  poly.map_coords(|&x| func.convert(x).unwrap())
}

/// convert XYZ tiles into lat/long.
/// this returns the NW / top left corner. Use x+1 and y+1 to get other corners.
/// this math was adapted from https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Python
/// FIXME:  lat is too high?
fn to_lng_lat(x:u32, y:u32, z: u8) -> Coordinate<f64> {
    let n = 2_f64.powi(z as i32);
    let lon = x as f64 / n * 360.0 - 180.0;
    let lat_rad = (PI * (1_f64 - 2_f64 * y as f64 / n)).sinh().atan();
    let lat = lat_rad * 180_f64 / PI;
    return Coordinate{x: lon, y: lat}
}

/// to_bounds returns the lat/lng tile boundaries (EPSG:4326)
/// for a tile from a URL with z/x/y format.
pub fn to_bounds(z: u8, x:u32, y:u32) -> Polygon<f64> {
    polygon!(
        to_lng_lat(x, y+1, z),
        to_lng_lat(x+1, y+1, z),
        to_lng_lat(x+1, y, z),
        to_lng_lat(x, y, z)
    )
}


/// get_tile_bounds returns the tile boundaries (web mercator EPSG:3857)
/// for a tile from a URL with z/x/y format.
pub fn web_mercator_tile_bounds(z: u8, x:u32, y:u32) -> Polygon<f64> {
  let grid = Grid::web_mercator();
  let extent = grid.tile_extent_xyz(x, y, z);
  println!("z:{}, x:{}, y:{}, extent: {:?}", z, x, y, extent);
  polygon!(
      (x: extent.minx, y: extent.miny),
      (x: extent.maxx, y: extent.miny),
      (x: extent.maxx, y: extent.maxy),
      (x: extent.minx, y: extent.maxy),
  )
}

mod tests {
  use super::*;
  #[test]
  fn test_to_lng_lat() {
      let ul = to_lng_lat(486, 332, 10);
      let cmp =  Coordinate{x:-9.140625, y:53.33087298301705};
      assert_eq!(true, (ul.x - cmp.x).abs() < 0.0000001);
      assert_eq!(true, (ul.y - cmp.y).abs() < 0.0000001);
  }
}


