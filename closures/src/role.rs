use std::cmp::Ordering;

#[derive(Debug)]
pub struct Hero {
    pub attack: u32,
    pub defense: u32,
}

// take a mutable slice as param
pub fn sort_roles(roles: &mut [Hero]) {
    let mut counter = 0;
    // sort_by_key want a FnMut closure
    //roles.sort_by_key(|h| {
    //    counter += 1;
    //    h.attack
    //});

    // compare the attack, if equal compare the defense
    // sort_by want a Ordering
    roles.sort_by(|a, b| {
        counter += 1;
        let rslt = a.attack.cmp(&b.attack);
        match rslt {
            Ordering::Equal => a.defense.cmp(&b.defense),
            _ => rslt,
        }
    });

    println!("List sorted by attack: {:?}", roles);
    println!("closure called {:?} times.", counter);
}
