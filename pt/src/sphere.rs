use math::{self, Norm, Point3f, Vector3f, Ray3f, Real};
use math::{Dot};
use {Surface, SurfacePoint, Bsdf};
use bsdf::BsdfRef;
use std::f64::consts::PI;
use color::{self, Color};
use std::sync::Arc;
use aabb::{Aabb3, HasBounds};

#[derive(Clone)]
pub struct Sphere {
    pub position: Point3f,
    pub radius: Real,
    pub bsdf: Arc<Bsdf>,
}

impl Sphere {
    pub fn new(position: Point3f, radius: Real, mat: Arc<Bsdf>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            bsdf: mat,
        }
    }

    pub fn bsdf<'s>(&'s self) -> BsdfRef<'s> {
        BsdfRef::Ref(self.bsdf.as_ref())
    }

    fn normal_to(&self, point: &Point3f) -> Vector3f {
        (*point - self.position).normalize()
    }
}

impl Surface for Sphere {

    fn is_emitter(&self) -> bool {
        if let Some(_) = self.bsdf.emittance() {
            true
        } else {
            false
        }
    }

    fn intersection (&self, ray: &Ray3f) -> Option<(Real, SurfacePoint)> {
        if let Some(t) = math::intersection_sphere(&self.position, self.radius, ray) {
            let pos = ray.origin + ray.dir * t;
            let norm = self.normal_to(&pos);
            Some((
                t,
                SurfacePoint {
                    position: pos,
                    normal: norm,
                    bsdf: self.bsdf(),
                    surface: self,
                }
            ))
        } else {
            None
        }
    }

    fn area (&self) -> Real {
        use std::f32::consts::PI;
        4.0 * (PI as Real) * self.radius * self.radius
    }

    fn total_emittance(&self) -> Option<Color> {
        if let Some(e) = self.bsdf.emittance() {
            Some(e * self.area() as f32)
        } else {
            None
        }
    }

    fn normal_at(&self, pos: &Point3f) -> Vector3f {
        self.normal_to(pos)
    }

    fn sample_surface_p(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
        let view_dir = (*view_point.0 - self.position).normalize();
        let normal = math::hs_uniform_sampling(&view_dir);
        // let normal = math::sph_uniform_sampling();
        let pdf = 2.0 / self.area();
        let pos = self.position + (normal * self.radius);

        (SurfacePoint {
            position: pos,
            normal: normal,
            bsdf: self.bsdf(),
            surface: self,
        },
        pdf)
    }

    fn pdf_p(&self, _: (&Point3f, &Vector3f), _ : (&Point3f, &Vector3f)) -> Real {      
        2.0 / self.area()
    }

    // fn sample_surface_d(&self, view_point: (&Point3f, &Vector3f)) -> (SurfacePoint, Real) {
    //     let view_dir = (*view_point.0 - self.position);
    //     let d = view_dir.norm();
    //     let r = self.radius;
    //     let normal = math::hs_uniform_sampling(&view_dir.normalize());    
    //     let pdf = 1.0 / (2.0 * (PI as Real) * (1.0 - d / (d*d + r*r).sqrt()));
    //     let pos = self.position + (normal * self.radius);

    //     (SurfacePoint {
    //         position: pos,
    //         normal: normal,
    //         bsdf: self.bsdf(),
    //         surface: self,
    //     },
    //     pdf)
    // }

    // fn pdf_d(&self, point_at_surface: (&Point3f, &Vector3f), view_point: (&Point3f, &Vector3f)) -> Real {
    //     let view_dir = (*view_point.0 - *point_at_surface.0);
    //     let d = view_dir.norm();
    //     let r = self.radius;
    //     let pdf = 1.0 / (2.0 * (PI as Real) * (1.0 - d / (d*d + r*r).sqrt()));
    //     pdf
    // }
}

impl HasBounds for Sphere {
    fn aabb(&self) -> Aabb3 {
        let dpos = Vector3f::new(self.radius, self.radius, self.radius);
        Aabb3::new(
            self.position - dpos,
            self.position + dpos,
        )
    }
}