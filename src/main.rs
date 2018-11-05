extern crate exec;

use std::f64;


fn get_x(latitude: f64, zoom_lev: f64) -> String{
    
     let raw_x:f64 =  2.0f64.powf( zoom_lev+7.0)*(latitude.to_radians()+ 1.0);

     let _x = raw_x as i64;
     let x = _x /256; 
     x.abs().to_string()
    

}

fn get_y(longtitude: f64, zoom_lev: f64) -> String{
     let raw_y:f64 = 2.0f64.powf( zoom_lev+7.0)*f64::consts::FRAC_1_PI*(-1.0*longtitude.to_radians().sin().atanh())+85.05112878f64.to_radians().sin().atanh();

     let _y = raw_y as i64;
     let y = _y /256;
     y.abs().to_string()
}


pub fn main() {
    
    let url =  format!("{}{}{}{}{}", "https://a.tiles.openstreetmap.org/13/", get_x(135.0, 49.0),"/", get_y(135.0, 49.0), ".png");
    println!("{}", url); 
    let get_https = exec::Command::new("curl")
        .arg("-o")
        .arg("test.png")
        .arg(url)
        .exec();

    println!("{}",get_https);
}

