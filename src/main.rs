use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, SystemTime};

const WIDTH:  usize = 640;
const HEIGHT: usize = 640;

const PI: f32 = std::f32::consts::PI;

struct Vec3
{
    x: f32,
    y: f32,
    z: f32
}

impl Vec3
{
    fn new() -> Vec3
    {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    fn from(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3{x: x, y: y, z: z}
    }

    fn sc_mul(&self, sc: f32) -> Vec3
    {
        return Vec3::from(
            self.x * sc,
            self.y * sc,
            self.z * sc
        )
    }

    fn add(&self, vec_b: &Vec3) -> Vec3
    {
        return Vec3::from(
            self.x + vec_b.x,
            self.y + vec_b.y,
            self.z + vec_b.z
        )
    }

    fn sub(&self, vec_b: &Vec3) -> Vec3
    {
        return Vec3::from(
            self.x - vec_b.x,
            self.y - vec_b.y,
            self.z - vec_b.z
        );
    }

    fn dot(&self, vec_b: &Vec3) -> f32
    {
        return self.x*vec_b.x + self.y*vec_b.y + self.z*vec_b.z;
    }

    fn length(&self) -> f32
    {
        return self.dot(self).sqrt();
    }

    fn normalize(&self) -> Vec3
    {
        let length = self.length();
        return Vec3::from(self.x / length, self.y / length, self.z / length);
    }
}

struct Ray
{
    origin:    Vec3,
    direction: Vec3
}

impl Ray
{
    fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray {
            origin: origin,
            direction: direction
        }
    }
}

struct Camera
{
    near: f32,
    far: f32,
    fov: f32
}

impl Camera
{
    pub fn new(far: f32, fov: f32) -> Camera
    {
        Camera {
            near: 1.0 / fov.tan(), 
            far:  far,
            fov:  fov
        }
    }
}

// Utilities 
fn color(r: u8, g: u8, b: u8, a: u8) -> u32
{
    let mut fin: u32 = (b as u32);
    fin += (g as u32) << 8;
    fin += (r as u32) << 8 * 2;
    fin += (a as u32) << 8 * 3;

    return fin;
}

fn normalize_coords(x: usize, y: usize) -> Vec3 
{
    let x = ((2 * x) as f32) / (WIDTH as f32);
    let y = ((2 * y) as f32) / (HEIGHT as f32);
    return Vec3::from(1.0 - x * (WIDTH as f32) / (HEIGHT as f32), 1.0 - y, 0.0);
}

// Casts the ray and returns the color
fn cast_ray(ray: Ray, ctime: f32) -> u32
{
    fn sphere(pos: &Vec3, R: f32, ray: Ray, ctime: f32) -> u32
    {
        let a = ray.direction.length().powf(2.0);
        let b = 2.0 * ray.origin.sub(pos).dot(&ray.direction);
        let c = pos.length().powf(2.0) + ray.origin.length().powf(2.0) - 2.0 * ray.origin.dot(pos) - R*R;
        let discriminant = b*b - 4.0*a*c;

        if discriminant > 0.0 {
            // Calculate normal and lighting
            let t0     = (b.powf(2.0) - discriminant.sqrt()) / (2.0 * a);
            let inter  = ray.direction.sc_mul(t0).add(&ray.origin);
            let normal = inter.sub(pos).normalize(); 

            let light_vec = Vec3::from(ctime.cos(), 0.0, ctime.sin()).normalize();
            let lighting_amount = light_vec.dot(&normal) * -1.0;
            if lighting_amount <= 0.0 { return color(0, 0, 0, 255); }

            return color((255.0 * lighting_amount.powf(2.0)) as u8, 0, 0, 255);
        }

        return color(0, 0, 0, 255);
    }

    return sphere(&Vec3::from(0.0, -2.0, 4.0), 0.75, ray, ctime);
    /*if amt >= 0.0 {
        return color((255.0 * amt) as u8, 0, 0, 255);
    }
    else {
        return color(0, 0, 0, 255);
    }*/
}

// Casts the array for the pixel
fn compute_pixel(x: usize, y: usize, camera: &Camera, ctime: f32) -> u32
{
    let coords = normalize_coords(x, y);
    let origin = Vec3::from(0.0, 0.0, -1.0 * camera.near);

    //if coords.sub(&Vec3::from(1.0, 1.0, 0.0)).length() <= 0.1 { return color(0, 0, 255, 255); }

    return cast_ray(
        Ray::new(
            origin,
            Vec3::from(coords.x, coords.y, camera.near)
        ),
        ctime
    );
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut current_time: f32 = 0.0;

    println!("{}", (WIDTH as f32) / (HEIGHT as f32));

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera: Camera = Camera::new(10.0, PI / 4.0);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let now = SystemTime::now();
    let mut i = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut index = 0;
        for i in buffer.iter_mut() {
            let x = index % WIDTH;
            let y = ((index as f32) / (HEIGHT as f32)).floor() as usize;
            *i = compute_pixel(x, y, &camera, current_time);
            
            index += 1;
        }

        current_time += 0.05;

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // Calculate FPS and write it to title
        let mut title: String = String::from("Test ");
        match now.elapsed() 
        {
            Ok(elapsed) => {
                let denom: f32 = elapsed.as_secs() as f32;
                title += &(((i as f32) / denom) as u32).to_string();
            } 
            Err(elapsed) => {
                title += &String::from("error");
            }
        }
        window.set_title(&title);
        i += 1;
    }
}
