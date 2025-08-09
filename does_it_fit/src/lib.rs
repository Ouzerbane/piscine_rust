mod areas_volumes;
pub use areas_volumes::{GeometricalShapes,GeometricalVolumes} ;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
     let mut ere = x * y ;
    let mut rt : f64 = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a,b) ,
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle =>areas_volumes::rectangle_area(a,b) as f64,
    };
    rt = rt * times as f64 ;
    if rt > ere as f64{
        return false ;
    }else {
        return true ;
    }
    // todo!()
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
     let mut ere = x * y *z;
    let mut rt : f64 = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a) ,
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a,b),
        GeometricalVolumes::TriangularPyramid =>areas_volumes::triangular_pyramid_volume(a as f64,b) as f64,
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a , b , c) as f64,

    };
    rt = rt * times as f64 ;
    if rt > ere as f64{
        return false ;
    }else {
        return true ;
    }
    // todo!()
}