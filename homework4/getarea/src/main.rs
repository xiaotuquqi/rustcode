fn main(){
     let r1: Rectangle= Rectangle{
         length : 2.0,
        width: 3.0
     };
     let  c1: Circle = Circle{
         radius: 2.0
     };
     println!("r1 's area : {:?}",r1.area());
     println!("c1 's area : {:?}",c1.area());

}

trait Method{
    fn area <T>(&self) ->T;
}

struct Rectangle{
    length: f64,
    width: f64

}

impl Method for Rectangle{
    fn area (&self) ->f64{
        self.length *self.width
    } 
}

struct Circle{
    radius: f64
}

impl Method for Circle{
    fn area (&self) ->f64{
        std::f64::consts::PI* self.radius *self.radius 
    }
}