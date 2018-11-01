extern crate exec;

fn get_x(latitude: f64, zoom_lev: f64){

    let x = 2.0f64.powf( zoom_lev+7.0)*(latitude/180.0 + 1.0);
    

}

fn get_y(longtitude: f64, zoom_lev: f64){
    let y = 2.0f64.powf( zoom_lev+7.0)*std::f64::consts::FRAC_1_PI*(-1.0*(0.5*((1.0+longtitude)/(1.0-longtitude))f64.log());
}


pub fn main() {

    let get_https = exec::Command::new("curl")
        .arg("-o")
        .arg("favicon.ico")
        .arg("https://kosenfesmap.maizuru-ct.ac.jp/favicon.ico")
        .exec();

    println!("{}", get_https);
}

