
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
            let result: Vec<String> = free_var(m).iter().filter(|x| x != &var).map(|x| x.clone()).collect::<Vec<String>>();
            result
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
            target
        }
        Lambda::App(box l1,box l2) => {
            let assgined_l1 = assign(l1,var_name.clone(), exp.clone());
            let assgined_l2 = assign(l2,var_name,exp);
            Lambda::App(
                box assgined_l1,
                box assgined_l2,
            )
        }
        Lambda::Abstruct(var,box m) if var == var_name => {
            Lambda::Abstruct(
                var_name,
                box m
            )
        },
        Lambda::Abstruct(var,box m) if var != var_name && !(free_var(&exp).contains(&var)) => {
            Lambda::Abstruct(
                var,
                box assign(m,var_name,exp)
            )
        }
        Lambda::Abstruct(var,box m) if var != var_name && (free_var(&exp).contains(&var))  => {
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
            assign(beta_reduction(m),x,n)
        }
        Lambda::App(box e1,box e2) => {
            Lambda::App(box beta_reduction(e1),box beta_reduction(e2))
        }
        Lambda::Abstruct(var,box m) => {
            Lambda::Abstruct(var,box beta_reduction(m))
        }
        _ => {
            exp
        }
    }
}

fn beta_reduction_multiple(exp: Lambda) -> Lambda {
    let b = beta_reduction(exp.clone());
    if b == exp {
        b
    } else {
        beta_reduction_multiple(b)
    }
}

fn gen_unique_type_var_name() -> String {
    nanoid::simple()
}

fn zero() -> Lambda {
    Lambda::Abstruct(
        "f".to_string(),
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::Term("z".to_string())
        )
    )
}

fn one() -> Lambda {
    Lambda::Abstruct(
        "f".to_string(),
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::App(
                box Lambda::Term("f".to_string()),
                box Lambda::Term("z".to_string()),
            )
        )
    )
}

fn two() -> Lambda {
    Lambda::Abstruct(
        "f".to_string(),
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::App(
                box Lambda::Term("f".to_string()),
                box Lambda::App(
                    box Lambda::Term("f".to_string()),
                    box Lambda::Term("z".to_string()),
                ),
            )
        )
    )
}

fn three() -> Lambda {
    Lambda::Abstruct(
        "f".to_string(),
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::App(
                box Lambda::Term("f".to_string()),
                box Lambda::App(
                    box Lambda::Term("f".to_string()),
                    box Lambda::App(
                        box Lambda::Term("f".to_string()),
                        box Lambda::Term("z".to_string()),
                    ),
                )
            )
        )
    )
}


fn succ() -> Lambda {
    Lambda::Abstruct(
        "n".to_string(),
        box Lambda::Abstruct(
            "f".to_string(),
            box Lambda::Abstruct(
                "z".to_string(),
                box Lambda::App(
                    box Lambda::Term("f".to_string()),
                    box Lambda::App(
                        box Lambda::App(
                            box Lambda::Term("n".to_string()),
                            box Lambda::Term("f".to_string()),
                        ),
                        box Lambda::Term("z".to_string())
                    )
                )
            )
        )
    )
}

fn add() -> Lambda {
    Lambda::Abstruct(
        "m".to_string(),
        box Lambda::Abstruct(
            "n".to_string(),
            box Lambda::App(
                box Lambda::App(
                    box Lambda::Term("m".to_string()),
                    box succ(),
                ),
                box Lambda::Term("n".to_string()),
            )
        )
    )
}

fn n(num: i64) -> Lambda {
    match num {
        0 => zero(),
        _ => {
            let init = Lambda::Term("z".to_string());
            let app = (0..num).fold(init,|acm,num|
                Lambda::App(
                    box Lambda::Term("f".to_string()),
                    box acm,
                )
            );
            Lambda::Abstruct(
                "f".to_string(),
                box Lambda::Abstruct(
                    "z".to_string(),
                    box app,
                )
            )
        }
    }
}

fn mul() -> Lambda {
    Lambda::Abstruct(
        "m".to_string(),
        box Lambda::Abstruct(
            "n".to_string(),
            box Lambda::App(
                box Lambda::App(
                    box Lambda::Term("m".to_string()),
                    box Lambda::App(
                        box add(),
                        box Lambda::Term("n".to_string()),
                    )
                ),
                box zero(),
            )
        )
    )
}

fn exp() -> Lambda {
    Lambda::Abstruct(
        "m".to_string(),
        box Lambda::Abstruct(
            "n".to_string(),
            box Lambda::App(
                box Lambda::App(
                    box Lambda::Term("m".to_string()),
                    box Lambda::App(
                        box mul(),
                        box Lambda::Term("n".to_string()),
                    )
                ),
                box one(),
            )
        )
    )
}

fn is_zero() -> Lambda {
    Lambda::Abstruct(
        "n".to_string(),
        box Lambda::App(
            box Lambda::App(
                box Lambda::Term("n".to_string()),
                box Lambda::Abstruct(
                    "x".to_string(),
                    box f(),
                ),
            ),
            box t(),
        ),
    )
}

fn alpha_equivalence(exp1: Lambda,exp2: Lambda) -> bool {
    match (exp1,exp2) {
        (Lambda::Abstruct(v1,box e1),Lambda::Abstruct(v2,box e2)) => {
            let y = gen_unique_type_var_name();
            let e1 = assign(e1, v1, Lambda::Term(y.clone()));
            let e2 = assign(e2, v2, Lambda::Term(y));
            alpha_equivalence(e1, e2)
        },
        (Lambda::Term(t1),Lambda::Term(t2)) => {
            t1 == t2
        },
        (Lambda::App(box e11,box e12),Lambda::App(box e21,box e22)) => {
            alpha_equivalence(e11, e21) && alpha_equivalence(e12, e22)
        },
        _ => false
    }
}

fn prod_n(n: i64) -> Lambda {
    let x = (1..=n).fold(Lambda::Term("P".to_string()),|sum,current|
        Lambda::App(
            box sum,
            box Lambda::Term(format!("x_{:?}",current)),
        )
    );
    let p_abs = Lambda::Abstruct(
        "P".to_string(),
        box x,
    );
    let abs = Lambda::Abstruct(
        format!("x_{:?}",n),
        box p_abs,
    );
    (1..n).fold(abs,|sum,current|
        Lambda::Abstruct(
            format!("x_{:?}",n - current),
            box sum
        )
    )
}

fn prod_n_i(n: i64,i: i64) -> Lambda {
    (0..n).fold(Lambda::Term(format!("x_{:?}",i)),|sum,current|
        Lambda::Abstruct(
            format!("x_{:?}",n - current),
            box sum
        )
    )
}

fn turing_y_combinator() -> Lambda {
    Lambda::App(
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::Abstruct(
                "x".to_string(),
                box Lambda::App(
                    box Lambda::Term("x".to_string()),
                    box Lambda::App(
                        box Lambda::App(
                            box Lambda::Term("z".to_string()),
                            box Lambda::Term("z".to_string()),
                        ),
                        box Lambda::Term("x".to_string()),
                    )
                )
            )
        ),
        box Lambda::Abstruct(
            "z".to_string(),
            box Lambda::Abstruct(
                "x".to_string(),
                box Lambda::App(
                    box Lambda::Term("x".to_string()),
                    box Lambda::App(
                        box Lambda::App(
                            box Lambda::Term("z".to_string()),
                            box Lambda::Term("z".to_string()),
                        ),
                        box Lambda::Term("x".to_string()),
                    )
                )
            )
        )
    )
}

fn pred() -> Lambda {
    Lambda::Abstruct(
        "n".to_string(),
        box Lambda::Abstruct(
            "f".to_string(),
            box Lambda::Abstruct(
            "x".to_string(),
                box Lambda::App(
                    box Lambda::App(
                        box Lambda::App(
                            box Lambda::Term("n".to_string()),
                            box Lambda::Abstruct(
                                "g".to_string(),
                                box Lambda::Abstruct(
                                    "h".to_string(),
                                    box Lambda::App(
                                        box Lambda::Term("h".to_string()),
                                        box Lambda::App(
                                            box Lambda::Term("g".to_string()),
                                            box Lambda::Term("f".to_string()),
                                        )
                                    )
                                )
                            )
                        ),
                        box Lambda::Abstruct(
                            "u".to_string(),
                            box Lambda::Term("x".to_string()),
                        )
                    ),
                    box Lambda::Abstruct(
                        "u".to_string(),
                        box Lambda::Term("u".to_string()),
                    )
                ),
            )
        )
    )
}

fn sub() -> Lambda {
    Lambda::Abstruct(
        "m".to_string(),
        box Lambda::Abstruct(
            "n".to_string(),
            box Lambda::App(
                box Lambda::App(
                    box Lambda::Term("m".to_string()),
                    box pred(),
                ),
                box Lambda::Term("n".to_string()),
            )
        )
    )
}

/*fn fact() -> Lambda {
    Lambda::App(
        box turing_y_combinator(),
        box Lambda::Abstruct(
            "fact".to_string(),
            box Lambda::Abstruct(
                "n".to_string(),
                box Lambda::App(
                    box Lambda::App(
                        box Lambda::App(
                            box cond(),
                            box Lambda::App(
                                box is_zero(),
                                box Lambda::Term("n".to_string())
                            ),
                        ),
                        box one(),
                    ),
                    box Lambda::App(
                        box Lambda::App(
                            box mul(),
                            box Lambda::Term("n".to_string()),
                        ),
                        box Lambda::App(
                            box Lambda::Term("fact".to_string()),
                            box Lambda::App(
                                box Lambda::App(
                                    box sub(),
                                    box Lambda::Term("n".to_string()),
                                ),
                                box one(),
                            )
                        ),
                    ),
                )
            )
        )
    )
}*/

fn main() {

}

#[test]
fn free_var_succ(){
    let var = free_var(&succ());
    let v: Vec<String> = vec![];
    assert_eq!(var,v)
}

#[test]
fn free_var_zero(){
    let var = free_var(&zero());
    let v: Vec<String> = vec![];
    assert_eq!(var,v)
}

#[test]
fn free_var_one(){
    let var = free_var(&one());
    let v: Vec<String> = vec![];
    assert_eq!(var,v)
}

#[test]
fn free_var_add(){
    let var = free_var(&add());
    let v: Vec<String> = vec![];
    assert_eq!(var,v)
}

#[test]
fn assign_test() {
    let abs = Lambda::Abstruct(
        "y".to_string(),
        box Lambda::App(
            box Lambda::Term("x".to_string()),
            box Lambda::Term("y".to_string()),
        )
    );
    let result = assign(abs.clone(), "x".to_string(),Lambda::App(
        box Lambda::Term("y".to_string()),
        box Lambda::Term("z".to_string()),
    ));

    let expected_result = Lambda::Abstruct(
        "b".to_string(),
        box Lambda::App(
            box Lambda::App(
                box Lambda::Term("y".to_string()),
                box Lambda::Term("z".to_string()),
            ),
            box Lambda::Term("b".to_string()),
        )
    );
    assert!(alpha_equivalence(result,expected_result));
}

#[test]
fn cond_true() {
    let b = Lambda::App(
        box Lambda::App(
            box Lambda::App(
                box cond(),
                box t(),
            ),
            box Lambda::Term("A".to_string()),
        ),
        box Lambda::Term("B".to_string()),
    );
    let b1 = beta_reduction(beta_reduction(beta_reduction(beta_reduction(beta_reduction(b)))));
    assert_eq!(b1,Lambda::Term("A".to_string()))
}

#[test]
fn cond_true_multiple() {
    let b = Lambda::App(
        box Lambda::App(
            box Lambda::App(
                box cond(),
                box t(),
            ),
            box Lambda::Term("A".to_string()),
        ),
        box Lambda::Term("B".to_string()),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(b1,Lambda::Term("A".to_string()))
}

#[test]
fn cond_false_multiple() {
    let b = Lambda::App(
        box Lambda::App(
            box Lambda::App(
                box cond(),
                box f(),
            ),
            box Lambda::Term("A".to_string()),
        ),
        box Lambda::Term("B".to_string()),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(b1,Lambda::Term("B".to_string()))
}

#[test]
fn cond_false() {
    let b = Lambda::App(
        box Lambda::App(
            box Lambda::App(
                box cond(),
                box f(),
            ),
            box Lambda::Term("A".to_string()),
        ),
        box Lambda::Term("B".to_string()),
    );
    let b1 = beta_reduction(beta_reduction(beta_reduction(beta_reduction(beta_reduction(b)))));
    assert_eq!(b1,Lambda::Term("B".to_string()))
}

#[test]
fn is_succ_zero_one() {
    let b = Lambda::App(
        box succ(),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(one(),b1);
}

#[test]
fn is_succ_one_two() {
    let b = Lambda::App(
        box succ(),
        box one(),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(two(),b1);
}


#[test]
fn is_succ_two_three() {
    let b = Lambda::App(
        box succ(),
        box two(),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(three(),b1);
}


#[test]
fn is_succ_succ_zero_two() {
    let two_succ = Lambda::App(
        box succ(),
        box Lambda::App(
            box succ(),
            box zero(),
        )
    );
    let b3 = beta_reduction_multiple(two_succ);
    assert_eq!(two(),b3);
}

#[test]
fn is_succ_succ_succ_zero_three(){
     let three_succ = Lambda::App(
         box succ(),
         box Lambda::App(
            box succ(),
            box Lambda::App(
                box succ(),
                box zero(),
            )
        ),
     );
    let b3 = beta_reduction_multiple(three_succ);
    assert_eq!(three(),b3);
}


#[test]
fn is_zero_normal_form() {
    let result = beta_reduction_multiple(zero());
    assert_eq!(result,zero());
}

#[test]
fn is_zero_add_zero_zero(){
    let b = Lambda::App(
        box Lambda::App(
            box add(),
            box zero(),
        ),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(zero(),b1);
}

#[test]
fn is_zero_add_one_one(){
    let b = Lambda::App(
        box Lambda::App(
            box add(),
            box zero(),
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(b);
    assert_eq!(one(),b1);
}

#[test]
fn is_one_add_zero_one(){
    let b = Lambda::App(
        box Lambda::App(
            box add(),
            box one(),
        ),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(one(),b1));
}

#[test]
fn is_one_add_one_two(){
    let b = Lambda::App(
        box Lambda::App(
            box add(),
            box one(),
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(two(),b1));
}

#[test]
fn is_one_add_one_add_one_three(){ 
    let b = Lambda::App(
        box Lambda::App(
            box add(),
            box Lambda::App(
                box Lambda::App(
                    box add(),
                    box one(),
                ),
                box one(),
            ), 
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(three(),b1));
}

#[test]
fn is_one_add_one_succ_succ_zero() {
    let two_succ = Lambda::App(
        box succ(),
        box Lambda::App(
            box succ(),
            box zero(),
        )
    );
    let two_add = Lambda::App(
        box Lambda::App(
            box add(),
            box one(),
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(two_succ);
    let b2 = beta_reduction_multiple(two_add);
    assert!(alpha_equivalence(b1,b2));
}

#[test]
fn succ_reduction_succ_zero_is_two() {
    let b = Lambda::App(
        box succ(),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b.clone());
    let b2 = Lambda::App(
        box succ(),
        box b1.clone(),
    );
    let b3 = beta_reduction_multiple(b2);
    assert!(alpha_equivalence(two(),b3));
}

#[test]
fn alpha_equivalence_test(){
    let exp1 = Lambda::Abstruct(
        "x".to_string(),
        box Lambda::Abstruct(
            "x".to_string(),
            box Lambda::Term("x".to_string())
        )
    );
    let exp2 = Lambda::Abstruct(
        "y".to_string(),
        box Lambda::Abstruct(
            "x".to_string(),
            box Lambda::Term("x".to_string())
        )
    );
     let exp3 = Lambda::Abstruct(
        "x".to_string(),
        box Lambda::Abstruct(
            "y".to_string(),
            box Lambda::Term("x".to_string())
        )
    );
    assert!(alpha_equivalence(exp1.clone(),exp2));
    assert!(!alpha_equivalence(exp1, exp3));
}

#[test]
fn n_0(){
    let z = n(0);
    assert!(alpha_equivalence(z,zero()));
}

#[test]
fn n_1(){
    let o = n(1);
    assert!(alpha_equivalence(o,one()));
}

#[test]
fn n_add_one_is_succ_n(){
    let n = n(100);
    let b1 = Lambda::App(
        box Lambda::App(
            box add(),
            box one(),
        ),
        box n.clone(),
    );
    let b2 = Lambda::App(
        box succ(),
        box n,
    );
    let b3 = beta_reduction_multiple(b1);
    let b4 = beta_reduction_multiple(b2);
    assert!(alpha_equivalence(b3, b4));
}

#[test]
fn zero_mul_zero(){
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box zero(),
        ),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, zero()));
}

#[test]
fn one_mul_two(){
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box one(),
        ),
        box two(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, two()));
}

#[test]
fn one_mul_zero() {
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box one(),
        ),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, zero()));
}

#[test]
fn two_mul_four() {
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box n(2),
        ),
        box n(4),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, n(8)));
}


#[test]
fn five_mul_ten() {
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box n(5),
        ),
        box n(10),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, n(50)));
}

#[test]
fn one_mul_one_mul_one(){
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box Lambda::App(
                box Lambda::App(
                    box mul(),
                    box one(),
                ),
                box one(),
            ), 
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(one(),b1));
}


#[test]
fn two_mul_two_mul_two(){
    let b = Lambda::App(
        box Lambda::App(
            box mul(),
            box Lambda::App(
                box Lambda::App(
                    box mul(),
                    box n(2),
                ),
                box n(2),
            ), 
        ),
        box n(2),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(n(8),b1));
}

#[test]
fn exp_one(){
    let b = Lambda::App(
        box Lambda::App(
            box exp(),
            box one(),
        ),
        box n(1),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(b1, one()));
}

#[test]
fn exp_two(){
    let b = Lambda::App(
        box Lambda::App(
            box exp(),
            box n(2),
        ),
        box n(1),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(b1, one()));
}

#[test]
fn is_zero_test(){
    let b = Lambda::App(
        box is_zero(),
        box zero(),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(b1, t()));

    let b = Lambda::App(
        box is_zero(),
        box n(100),
    );
    let b1 = beta_reduction_multiple(b.clone());
    assert!(alpha_equivalence(b1, f()));
}

#[test]
fn prod_n_test_three(){
    let p = prod_n(3);
    let expected_result = Lambda::Abstruct(
        "x_1".to_string(),
        box Lambda::Abstruct(
           "x_2".to_string(),
           box Lambda::Abstruct(
               "x_3".to_string(),
               box Lambda::Abstruct(
                   "P".to_string(),
                    box Lambda::App(
                        box Lambda::App(
                            box Lambda::App(
                                box Lambda::Term("P".to_string()),
                                box Lambda::Term("x_1".to_string()),  
                            ),
                            box Lambda::Term("x_2".to_string()),
                        ),
                        box Lambda::Term("x_3".to_string()),
                    )
               )
           )
        )
    );
    assert_eq!(p,expected_result);
}

#[test]
fn prod_n_test_one(){
    let p = prod_n(1);
    let expected_result = Lambda::Abstruct(
        "x_1".to_string(),
        box Lambda::Abstruct(
            "P".to_string(),
            box Lambda::App(
                box Lambda::Term("P".to_string()),
                box Lambda::Term("x_1".to_string()),
            )
        )
    );
    assert_eq!(p,expected_result);
}

#[test]
fn prod_n_i_test(){
    let p = prod_n_i(2,1);
    let expected_result = Lambda::Abstruct(
        "x_1".to_string(),
        box Lambda::Abstruct(
            "x_2".to_string(),
            box Lambda::Term("x_1".to_string()),
        )
    );
    assert_eq!(p,expected_result);
}


#[test]
fn prod_n_i_test2(){
    let p = prod_n_i(2,2);
    let expected_result = Lambda::Abstruct(
        "x_1".to_string(),
        box Lambda::Abstruct(
            "x_2".to_string(),
            box Lambda::Term("x_2".to_string()),
        )
    );
    assert_eq!(p,expected_result);
}

#[test]
fn prod_n_i_test3(){
    let p = prod_n_i(5,2);
    let expected_result = Lambda::Abstruct(
        "x_1".to_string(),
        box Lambda::Abstruct(
            "x_2".to_string(),
            box Lambda::Abstruct(
                "x_3".to_string(),
                box Lambda::Abstruct(
                    "x_4".to_string(),
                    box Lambda::Abstruct(
                        "x_5".to_string(),
                        box Lambda::Term("x_2".to_string()),
                    )
                )
            )
        )
    );
    assert_eq!(p,expected_result);
}


#[test]
fn prod_test(){
    let p = Lambda::App(
        box Lambda::App(
            box prod_n(2),
            box Lambda::Term("M_1".to_string()),
        ),
        box Lambda::Term("M_2".to_string()),
    );
    let b = Lambda::App(
        box p,
        box prod_n_i(2,1),
    );
    let b1 = beta_reduction_multiple(b);
    println!("{:?}",b1);
    assert!(alpha_equivalence(b1,Lambda::Term("M_1".to_string())));
}


#[test]
fn is_pred_one_zero() {
    let b = Lambda::App(
        box pred(),
        box one(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(zero(),b1));
}

#[test]
fn is_pred_two_one() {
    let b = Lambda::App(
        box pred(),
        box two(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(one(),b1));
}

#[test]
fn one_sub_one_zero(){
    let b = Lambda::App(
        box Lambda::App(
            box sub(),
            box one(),
        ),
        box one(),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(zero(),b1));
}

#[test]
fn five_sub_one_four(){
    let b = Lambda::App(
        box Lambda::App(
            box sub(),
            box one(),
        ),
        box n(5),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(n(4),b1));
}

#[test]
fn five_sub_three_two(){
    let b = Lambda::App(
        box Lambda::App(
            box sub(),
            box n(3),
        ),
        box n(5),
    );
    let b1 = beta_reduction_multiple(b);
    assert!(alpha_equivalence(n(2),b1));
}