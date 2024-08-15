fn lemonade_change(bills: Vec<i32>) -> bool {
    let (mut f_cents, mut t_cents) = (0, 0);
    for i in bills {
        if i == 5 {
            f_cents += 1;
        }
        else if i == 10 {
            f_cents -= 1;
            t_cents += 1;
        }
        else if i == 20 {
            if t_cents > 0 {
                f_cents -= 1;
                t_cents -= 1;
            } else {
                f_cents -= 3;
            }
            
        }

        if f_cents < 0 || t_cents < 0 {
            return false;
        }
    }
    true
}

fn main() {
    let a = lemonade_change(vec![5,5,10,20,5,5,5,5,5,5,5,5,5,10,5,5,20,5,20,5]);
    println!("{}", a);
}
