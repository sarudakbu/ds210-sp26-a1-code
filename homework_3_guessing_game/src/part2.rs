use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut high = max - 1;
        let mut low = min;
        
        while low < high{
            let mid = (low + high) / 2;
            
            match player.ask_to_compare(mid){
            1 => low = mid + 1,
            0 => return mid,
            -1 => high = mid -1,
            _ => panic!("No solution provided yet")
               }
        }

        low
    }
}

