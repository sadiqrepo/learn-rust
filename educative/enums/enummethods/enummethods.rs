
#![allow(dead_code)]
#[derive(Debug)]

enum TrafficSignal {
    Red, Yellow, Green
}

impl TrafficSignal {

    fn is_stop(&self)-> bool {
        match self {
            TrafficSignal::Red=> return true,
            _ => return false
        }
    }
}

fn main(){
    let action = TrafficSignal::Green;
    println!("What is the sign on Traffic signal? {:?}",action);
    println!("Should I Stop? {}",action.is_stop());
}
