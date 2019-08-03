Day 68: Gale-Shapley
<br>
The Stable Marriage Problem states that given N men and N women, where each person has ranked all members of the opposite sex in order of preference, marry the men and women together such that there are no two people of opposite sex who would both rather have each other than their current partners. If there are no such people, all the marriages are “stable” (Source Wiki).
<br>
Consider the following example.
<br>
```
Let there be two men m1 and m2 and two women w1 and w2.
Let m1‘s list of preferences be {w1, w2}
Let m2‘s list of preferences be {w1, w2}
Let w1‘s list of preferences be {m1, m2}
Let w2‘s list of preferences be {m1, m2}
```

<br>
The matching { {m1, w2}, {w1, m2} } is not stable because m1 and w1 would prefer each other over their assigned partners. The matching {m1, w1} and {m2, w2} is stable because there are no two people of opposite sex that would prefer each other over their assigned partners.
<br>
It is always possible to form stable marriages from lists of preferences (See references for proof). Following is Gale–Shapley algorithm to find a stable matching:
The idea is to iterate through all free men while there is any free man available. Every free man goes to all women in his preference list according to the order. For every woman he goes to, he checks if the woman is free, if yes, they both become engaged. If the woman is not free, then the woman chooses either says no to him or dumps her current engagement according to her preference list. So an engagement done once can be broken if a woman gets better option. Time Complexity of Gale-Shapley Algorithm is O(n2).
Following is complete algorithm from Wiki
<br>

```
Initialize all men and women to free
while there exist a free man m who still has a woman w to propose to 
{
    w = m's highest ranked such woman to whom he has not yet proposed
    if w is free
       (m, w) become engaged
    else some pair (m', w) already exists
       if w prefers m to m'
          (m, w) become engaged
           m' becomes free
       else
          (m', w) remain engaged    
}
```
run:
```
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

```
result:
```
woman:     man:      
diana      charlie   
betty      bob       
alice      adam      
claire     david     

```
