
extern crate png;
extern crate rand;

mod math3d;
mod scene_objects;
use math3d::Vec3;

fn push_normal( image_data: &mut Vec<u8>, n:&Vec3) {
    let normalized = (n + Vec3::new(1.0, 1.0, 1.0))* 0.5;
    push_color( image_data, &normalized);
}

fn push_color( image_data: &mut Vec<u8>, col:&Vec3) {
    image_data.push( ( col.x * 255.0) as u8);
    image_data.push( ( col.y * 255.0) as u8);
    image_data.push( ( col.z * 255.0) as u8);
    image_data.push( 255 );
}

fn ambient_occlusion(objects: &Vec<Box<scene_objects::Object3D>>, pos: &Vec3, normal: &Vec3, rng: &mut rand::Rng, num_samples: u32, distance_cutoff: f64) -> f64 {
    let mut num_hits = 0;
    for _ in 0..num_samples {
        let d = normal.get_cosine_distributed_random_ray(rng);
        if let Some(hit) = trace_ray( &objects, &pos, &d) {
            if hit.distance < distance_cutoff {
                num_hits += 1;
            }
        }
    }

    1.0 - (num_hits as f64) / (num_samples as f64)
}

fn trace_ray<'a>(objects: &'a Vec<Box<scene_objects::Object3D>>, ray_src: &Vec3, ray_dir: &Vec3) -> Option<scene_objects::HitRecord<'a>>
{
    //println!("tracing ray from {} with {}", ray_src, ray_dir);
    let mut hit_obj: Option<scene_objects::HitRecord> = None;

    for obj in objects.iter() {
        if let Some(hit) = obj.hit(&ray_src, &ray_dir) {
            //println!("Hit obj {} at {}", hit.object.get_color(), (ray_src + ray_dir*hit.distance));
            if let Some(old_hit) = hit_obj {
                if hit.distance < old_hit.distance {
                    hit_obj = Some(hit);
                } else {
                    hit_obj = Some(old_hit);
                }
            } else {
                hit_obj = Some(hit)
            }
        }
    }

    hit_obj
}

fn clamp(v: f64, min: f64, max: f64) -> f64{
    if v < min {
        return min;
    }

    if v > max {
        return max
    }

    v
}

fn main() {

    // For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // To use encoder.set()
    use png::HasParameters;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1023, 767);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut image_data: Vec<u8> = Vec::new();

    let mut objects: Vec<Box<scene_objects::Object3D>> = Vec::new();
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(-100.0, -80.0, 400.0),
        40.0,
        Vec3::new(0.8, 0.8, 0.8),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(100.0, -80.0, 400.0),
        40.0,
        Vec3::new(0.8, 0.8, 0.8),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(0.0, 0.0, 700.0),
        350.0,
        Vec3::new(0.8, 0.8, 0.0),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(100.0, -80.0, 370.0),
        20.0,
        Vec3::new(0.1, 0.1, 0.1),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(-100.0, -80.0, 370.0),
        20.0,
        Vec3::new(0.1, 0.1, 0.1),
    )));

    objects.push(Box::new(scene_objects::Plane::new(
        Vec3::new(0.0, 200.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.1, 0.5, 0.1),
    )));
    let mut rng = rand::XorShiftRng::new_unseeded();
    let ambient = 0.3;

    let ray_src = Vec3::new(0.0, 0.0, 0.0);
    let light_dir = Vec3::new(-1.0, -1.0, -1.0).normalized();

    for y in -383i16..384 {
        for x in -511i16..512 {
//    let y = -283;
//    let x = 0;
//    {{
            let v = Vec3::new(x as f64, y as f64, 512.0).normalized();
            let mut hit_obj = trace_ray(&objects, &ray_src, &v);

            if let Some(mut obj) = hit_obj {
                let p_hit = &ray_src + &v * obj.distance;
                let n = obj.object.normal_at(p_hit);
                let p_hit = p_hit + n * 1E-7;
                //let r = v.reflect_at(&n);

                //push_color( &mut image_data, &Vec3::new(f, f, f));

                let diffuse = Vec3::dot(&n, &light_dir);
                let diffuse = match diffuse > 0.0 {
                    true => diffuse,
                    false => 0.0
                };

                let brightness = (diffuse + ambient) * ambient_occlusion(&objects, &p_hit, &n, &mut rng, 1000, 200.0);
                let brightness = clamp(brightness, 0.0, 1.0);

                let col: &Vec3 = &obj.object.get_color();
                let col = col * brightness;
                //let col = Vec3::new(1.0, 1.0, 1.0)* ambient_occlusion(&objects, &p_hit, &n, &mut rng, 1000, 200.0);
                push_color( &mut image_data, &col);
                //push_normal( &mut image_data, &r);
                //}

            } else {
                push_normal(&mut image_data, &v);
            }
        }
    }
    writer.write_image_data(&image_data).unwrap(); // Save
}
