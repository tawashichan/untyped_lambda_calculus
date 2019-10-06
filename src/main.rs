
#![feature(slice_patterns)]
#![feature(box_syntax, box_patterns)]
#![feature(vec_remove_item)]

mod ast;

use ast::{Lambda,Var};
use nanoid;

fn t() -> Lambda {
    Lambda::Abstruct("x".to_string(),box Lambda::Abstruct("y".to_string(),box Lambda::Term("x".to_string())))
}

fn f() -> Lambda {
    Lambda::Abstruct("x".to_string(),box Lambda::Abstruct("y".to_string(),box Lambda::Term("y".to_string())))
}

fn cond() -> Lambda {
    Lambda::Abstruct("b".to_string(),
        box Lambda::App(
            box Lambda::App(
                box Lambda::Term("b".to_string()),
                box Lambda::Term("A".to_string())
            ),
            box Lambda::Term("B".to_string())
        )
    )
}

fn cond_var() -> Lambda {
    Lambda::Abstruct("b".to_string(),
        box Lambda::Abstruct("A".to_string(),
            box Lambda::Abstruct("B".to_string(),
                box Lambda::App(
                    box Lambda::App(
                        box Lambda::Term("b".to_string()),
                        box Lambda::Term("A".to_string()),
                    ),
                    box Lambda::Term("B".to_string())
                )
            )
        ),
    )
}

fn free_var(exp: &Lambda) -> Vec<Var>{
    match exp {
        Lambda::Term(var) => vec![var.clone()],
        Lambda::App(box e1,box e2) => {
            let mut e1_free_var = free_var(e1);
            e1_free_var.append(&mut free_var(e2));
            e1_free_var
        }
        Lambda::Abstruct(var,box m) => {
            let mut m_free_var = free_var(m);
            m_free_var.remove_item(var);
            m_free_var
        }
        _ => vec![]
    }
}

fn assign(target: Lambda,var_name: String,exp: Lambda) -> Lambda {
    match target {
        Lambda::Term(var) if var_name == var => {
            exp
        } 
        Lambda::Term(_) => {
            target.clone()
        }
        Lambda::App(box l1,box l2) => {
            let assgined_l1 = assign(l1,var_name.clone(), exp.clone());
            let assgined_l2 = assign(l2,var_name.clone(), exp.clone());
            Lambda::App(
                box assgined_l1,
                box assgined_l2,
            )
        }
        Lambda::Abstruct(var,box m) if var == var_name => {
            Lambda::Abstruct(
                var_name.clone(),
                box m.clone()
            )
        },
        Lambda::Abstruct(var,box m) if !(free_var(&exp).contains(&var)) => {
            Lambda::Abstruct(
                var.clone(),
                box assign(m,var_name,exp)
            )
        }
         Lambda::Abstruct(var,box m) => {
            let z = gen_unique_type_var_name();
            Lambda::Abstruct(
                z.clone(),
                box assign(assign(m,var,Lambda::Term(z)),var_name,exp)
            )
        }
        _ => {
            unimplemented!()
        }
    }
}

fn beta_reduction(exp: Lambda) -> Lambda {
    match exp {
        Lambda::App(box Lambda::Abstruct(x,box m),box n) => {
            assign(m,x,n)
        }
        Lambda::App(box e1,box e2) => {
            beta_reduction(Lambda::App(box beta_reduction(e1),box e2))
        }
        _ => {
            exp
        }
    }
}

fn gen_unique_type_var_name() -> String {
    nanoid::simple()
}

fn main() {
    let b = Lambda::App(
        box cond_var(),
        box t(),
    );
    let b = Lambda::App(
        box Lambda::App(
            box Lambda::App(
                box cond_var(),
                box t(),
            ),
            box Lambda::Term("A".to_string()),
        ),
        box Lambda::Term("B".to_string()),
    );
    let b1 = beta_reduction(b);
    println!("{:?}",b1);
    let b2 = beta_reduction(b1);
    println!("{:?}",b2);
}
