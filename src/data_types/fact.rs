use std::{collections::HashMap, hash::RandomState, string};

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

pub struct Requirement {
    pub symbol: String,       //A => [!A, A, A, !A]
    pub condition: Condition, //   [false, true, true, false]
    pub not: bool,
}

impl Requirement {
    pub fn new(symbol: String, condition: Condition, not: bool) -> Self {
        Requirement {
            symbol,
            condition,
            not,
        }
    }
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

pub struct Knowledge {
    pub symbol: String, //(!(E + F) ^ G) => !A
    pub fact: bool,
    pub requirements: Vec<Requirement>, //E AND F
    pub not: bool,
}

//(E + F) not true
//E AND F

//(!(E + F) ^ G) not false

//(!(E + F) ^ G)
//
//

