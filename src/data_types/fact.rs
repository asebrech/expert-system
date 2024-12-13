use std::{collections::HashMap, hash::RandomState};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Condition {
    START,
    AND,
    OR,
    XOR,
    END,
}

//knowledge = A + (E + F) = V
//V
//requiert :
// A +
// (E + F)


// A CHECK PLUS EN PROFONDEUR
// check apres, si tu as l un tu n as pas l autre et vice vers ca
// C | D => X | V
// X + V = Y
// Y = X + V
// !A <=> A   <=== impossible, contradiction

//C => !X | V
//

// A => B    <== true
// C => B    <=== false

//implique 
//A + B <=> C
//A + B = C
//C = A + B


//=A
//A => B ^ C

pub struct Requirement<'a> {
    pub knowledge: &'a Knowledge<'a>, 
    pub condition: Condition,
    pub should_exist: bool,
}
//=YB
//Y => C
//B <=> A | C + Y
//A | C + Y => B

//A | C + Y + A => A
//A | C + Y + C => C
//A | C + Y + Y => Y
// combine B and A | C + Y et check si A existe
//(B) + (A | C + Y) + A => A


pub struct Knowledge<'a> {
    pub symbol: &'a str, //(E + F)   (!(E + F) ^ G)
    pub fact: bool,
    pub requirements: Vec<Requirement<'a>>, //E AND F
}


//(!(E + F) ^ G)