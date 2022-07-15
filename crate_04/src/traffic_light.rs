use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait TrafficLightDuration {
    fn get_duration(light: TrafficLight) -> Duration {
        match light {
            TrafficLight::Red => Duration::from_secs(10),
            TrafficLight::Green => Duration::from_secs(20),
            TrafficLight::Yellow => Duration::from_secs(5)            
        }
    }
}

pub struct  ShangHai
{

}
impl TrafficLightDuration for ShangHai {
    
}

fn main() {    
    let g = TrafficLight::Green;        
    let r = TrafficLight::Red;        
    let y = TrafficLight::Yellow;        
    println!("light: {:#?}  duration: {:#?}" , g, ShangHai::get_duration(g));
    println!("light: {:#?}  duration: {:#?}" , r, ShangHai::get_duration(r));
    println!("light: {:#?}  duration: {:#?}" , y, ShangHai::get_duration(y));
}
