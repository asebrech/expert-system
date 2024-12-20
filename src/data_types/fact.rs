#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Condition {
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

#[derive(Clone, Debug)]
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

// impl fmt::Debug for Requirement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{{ not: {}, symbol: {}, condition: {:?} }}",
//             self.not, self.symbol, self.condition
//         )
//     }
// }
//=YB
//Y => C
//B <=> A | C + Y
//A | C + Y => B

//A | C + Y + A => A
//A | C + Y + C => C
//A | C + Y + Y => Y
// combine B and A | C + Y et check si A existe
//(B) + (A | C + Y) + A => A

#[derive(Clone, Debug)]
pub struct Knowledge {
    pub symbol: String, //(!(E + F) ^ G) => !A
    pub fact: bool,
    pub requirements: Vec<Requirement>, //E AND F
    pub not: bool,
}

impl Knowledge {
    pub fn new(symbol: String, fact: bool, requirements: Vec<Requirement>, not: bool) -> Self {
        Knowledge {
            symbol,
            fact,
            requirements,
            not,
        }
    }
}

// impl fmt::Debug for Knowledge {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{{ not: {}, symbol: {}, fact: {}, requirements: {:?} }}",
//             self.not, self.symbol, self.fact, self.requirements
//         )
//     }
// }

//(E + F) not true
//E AND F

//(!(E + F) ^ G) not false

//(!(E + F) ^ G)
//
//
