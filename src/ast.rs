pub type Var = String;

pub type Assignment = (String,Lambda);

#[derive(Clone,Debug,PartialEq)]
pub enum Lambda {
    Term(Var),
    Abstruct(Var,Box<Lambda>),
    App(Box<Lambda>,Box<Lambda>),
}

