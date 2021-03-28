fn main() {
let  v: &[u32];
v = &[1, 4294967295, 3];
print!("{:?}",sum(v).unwrap())
}

fn sum( v:&[u32]) ->Option<u32>{
    let mut uu: u32 = 0;
    for i in v{
        if  uu+i>4294967295{
            return   None
           }
        uu = uu +i ;

            
    }   
    Some(uu)
}
