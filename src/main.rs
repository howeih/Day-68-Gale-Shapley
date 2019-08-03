use std::collections::VecDeque;
use std::collections::HashMap;

fn stable_match<'a>(men: &HashMap<&'a str, Vec<&'a str>>, women: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Option<&'a str>> {
    let mut free_man: VecDeque<&str> = men.keys().cloned().collect();
    let mut engaged = HashMap::<&str, Option<&str>>::new();
    while !free_man.is_empty() {
        let i = free_man.pop_front().unwrap();
        for &j in men.get(i).unwrap() {
            let preference = women.get(j).unwrap();
            let fiance = engaged.entry(j).or_insert(None);
            match fiance {
                Some(i) => {
                    if preference.iter().find(|&&x| *x == **i) >= preference.iter().find(|&&x| *x == **i) {
                        continue;
                    }
                    free_man.push_back(i);
                }
                None => ()
            }
            engaged.insert(j, Some(i));
            break;
        }
    }
    engaged
}

fn main() {
    let mut men = HashMap::<&str, Vec<&str>>::new();
    let mut women = HashMap::<&str, Vec<&str>>::new();
    men.insert("charlie", vec!["betty", "diana", "claire", "alice"]);
    men.insert("bob", vec!["betty", "claire", "alice", "diana"]);
    men.insert("adam", vec!["diana", "alice", "betty", "claire"]);
    men.insert("david", vec!["claire", "alice", "diana", "betty"]);
    women.insert("alice", vec!["david", "adam", "charlie", "bob"]);
    women.insert("betty", vec!["adam", "charlie", "bob", "david"]);
    women.insert("claire", vec!["adam", "bob", "charlie", "david"]);
    women.insert("diana", vec!["david", "adam", "charlie", "bob"]);
    let matching = stable_match(&men, &women);
    println!("{:<10} {:<10}", "woman:", "man:");
    for (&woman, &man) in &matching {
        println!("{:<10} {:<10}", woman, man.unwrap());
    }
}
