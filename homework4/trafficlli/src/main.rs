enum TrafficLIc{
    Red,
    Yellow,
    Green,
}

trait Traffic{
    fn time(&self) -> u8;
}

impl Traffic for TrafficLIc{
    fn time(&self) ->u8{
        match  &self{
            TrafficLIc::Red => 10u8,
            TrafficLIc::Yellow => 5u8,
            TrafficLIc::Green=> 20u8,
            
        }
        }
    }
    fn main(){
        let li1 = TrafficLIc::Red;
        let li2 = TrafficLIc::Yellow;
        let li3 = TrafficLIc::Green;
        println!("the green light is {:?} s",li3.time());
        println!("the red light is {:?} s",li1.time());
        println!("the yellow light is {:?} s",li2.time());
    }
    