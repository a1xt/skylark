use math::{intersection_with_sphere, BaseFloat, Norm, Point3f, Vector3f, Ray3f, Coord, Dot};
use super::{Surface, SurfacePoint, Material};
use std::boxed::Box;
use std::f32::consts::PI;
use math;
use utils::consts;

//#[derive(Clone)]
pub struct Sphere {
    pub position: Point3f,
    pub radius: Coord,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(position: Point3f, radius: Coord, mat: Box<Material>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            material: mat,
        }
    }

    pub fn material(&self) -> &Material {
        self.material.as_ref()
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
        //(self.position - *point).normalize()
    }
}

impl Surface for Sphere {

    fn intersection (&self, ray: &Ray3f) -> Option<(Coord, SurfacePoint)> {
        if let Some(t) = intersection_with_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    material: self.material(),
                    surface: self,
                }
            ))
        } else {
            None
        }
    }

    // fn random_point (&self) -> SurfacePoint {
    //     let norm = math::sph_uniform_sampling();
    //     let pos: Point3f = self.position +  (norm * self.radius);
        
    //     SurfacePoint {
    //         position: pos,
    //         //position: self.position,
    //         normal: norm,
    //         material: self.material(),
    //     }
    // }

    fn area (&self) -> Coord {
        use std::f32::consts::PI;
        4.0 * (PI as Coord) * self.radius * self.radius
    }

    fn normal_at(&self, pos: &Point3f) -> Vector3f {
        self.normal_to(pos)
    }

    fn sample_surface(&self, view_point: &Point3f) -> (SurfacePoint, Coord) {
        let view_dir = (*view_point - self.position).normalize();
        // let normal = math::sph_uniform_sampling(); 
        // let pdf = 1.0 / self.area();
        let normal = math::hs_cosine_sampling(&view_dir);
        let pos = self.position + (normal * self.radius);
        let cos_theta = normal.dot(&view_dir).abs();
        let pdf = cos_theta / ((PI as Coord) * self.radius * self.radius);
        

        (SurfacePoint {
            position: pos + normal * consts::POSITION_EPSILON,
            normal: normal,
            material: self.material(),
            surface: self,
        },
        pdf)
    }

    fn pdf(&self, view_point: &Point3f, point_at_surface: &Point3f) -> Coord {
        //1.0 / self.area()
        let view_dir = (*view_point - self.position).normalize();
        let normal = self.normal_to(point_at_surface);
        let cos_theta = normal.dot(&view_dir);
        cos_theta / (PI as Coord * self.radius * self.radius)
    }

}