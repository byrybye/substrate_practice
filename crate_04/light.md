```
use std::time::Duration;

//交通信号灯
#[derive(Debug, Clone, Copy)]
pub enum TrafficLight {
    Red, //红灯
    Green,//绿灯
    Yellow,//黄灯
}

//计算信号灯 持续时间
pub trait TrafficLightDuration {
    fn get_duration(light: TrafficLight) -> Duration {
        //使用 模式匹配处理 
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
```

![image]https://github.com/byrybye/substrate_practice/blob/main/crate_04/light.png
