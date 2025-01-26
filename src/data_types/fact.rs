// The START condition was removed because each requirement is either followed by another or is at the end.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Condition {
    AND,
    OR,
    XOR,
    END,
}

#[derive(Clone, Debug)]
pub struct Requirement {
    pub symbol: String,
    pub condition: Condition,
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

#[derive(Clone, Debug)]
pub struct Knowledge {
    pub symbol: String,
    pub fact: bool,
    pub calcul: String,
    pub requirements: Vec<Requirement>,
    pub result_requirement: Option<Vec<Requirement>>,
    pub not: bool,
}

impl Knowledge {
    pub fn new(
        symbol: String,
        fact: bool,
        calcul: String,
        requirements: Vec<Requirement>,
        result_requirement: Option<Vec<Requirement>>,
        not: bool,
    ) -> Self {
        Knowledge {
            symbol,
            fact,
            calcul,
            requirements,
            result_requirement,
            not,
        }
    }
}
