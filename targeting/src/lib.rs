use entities;

use entities::fighter::Fighter;

/*
    TODO: refactor targetting functions to:
        - return a usize instead of a fighter
        - use closures to determine target
        - rethink borrowing/ownership
 */
pub fn target_lowest_hp<'a>(fighters: Vec<&'a mut Fighter>) -> Option<&'a mut Fighter>
{
    fighters.into_iter()
        .filter(|f| f.is_alive())
        .min_by_key(|f| f.hp)
}

pub fn target_highest_hp<'a>(fighters: Vec<&'a mut Fighter>) -> Option<&'a mut Fighter>
{
    fighters.into_iter()
        .filter(|f| f.is_alive())
        .max_by_key(|f| f.hp)
}