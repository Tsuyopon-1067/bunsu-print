extern crate rand;

use rand::Rng;

fn main() {
    for _i in 1 .. 10 {
        let t = kakezan_item();
        println!("{}{}", t.0, t.1);
    }
}

// 約分問題1つを生成
fn yakubun_item() -> (String, String) {
    let mut a: i32;
    let mut b: i32;
    (a, b) = create_bunsu(14);
    
    let k = get_rand(2, 10);

    let res1: String = format!("\t\t\t\\item $\\displaystyle \\frac{{{}}}{{{}}}$", (b*k).to_string(), (a*k).to_string());
    let res2: String = format!("\t\t\t\\item $\\displaystyle \\frac{{{}}}{{{}}}$", b.to_string(), a.to_string());
    (res1, res2)
}

// 掛け算問題1つを生成
fn kakezan_item() -> (String, String) {
    let a: i32;
    let b: i32;
    (a, b) = create_bunsu(9);
    let c: i32;
    let d: i32;
    (c, d) = create_bunsu(9);

    let mut e: i32 = a * c;
    let mut f: i32 = b * d;
    (e, f) = yakubun(e, f);
    
    let res1: String = format!("\t\t\t\\item $\\displaystyle \\frac{{{}}}{{{}}} \\times \\frac{{{}}}{{{}}}$", b.to_string(), a.to_string(), d.to_string(), c.to_string());
    //let res1: String = format!("\t\t\t\\item $\\displaystyle {} \\times {}$", bunsu_text(0, 0, 0), bunsu_text(0, 0, 0));
    let res2: String = format!("\t\t\t\\item $\\displaystyle \\frac{{{}}}{{{}}}$", f.to_string(), e.to_string());
    (res1, res2)
}

// 割り算問題1つを生成
fn warizan_item() -> (String, String) {
    let a: i32;
    let b: i32;
    (a, b) = create_bunsu(9);
    let c: i32;
    let d: i32;
    (c, d) = create_bunsu(9);

    let mut e: i32 = a * d;
    let mut f: i32 = b * c;
    (e, f) = yakubun(e, f);
    
    let t1 = taibunsu(a, b);
    let res1: String = format!("\t\t\t\\item $\\displaystyle {} \\div {}$", bunsu_text(t1), bunsu_text(taibunsu(c, d)));
    let res2: String = format!("\t\t\t\\item $\\displaystyle \\frac{{{}}}{{{}}}$", f.to_string(), e.to_string());
    (res1, res2)
}

fn bunsu_text(t: (i32, i32, i32)) -> String {
    let mut res = String::new();
    if t.2 != 0 {
        res += &t.2.to_string();
        res += &" ";
    }
    res += &format!("\\frac{{{}}}{{{}}}", t.0, t.1);
    res
}
// c + b/a
fn taibunsu(a: i32, mut b: i32) -> (i32, i32, i32) {
    // b/a -> c + b'/a'
    let c = b / a;
    b = b % a;    
    (a, b, c)
}
// b/a
fn create_bunsu(n: i32) -> (i32, i32) {
    let mut a: i32 = get_rand(1, n);
    let mut b: i32 = get_rand(1, n);

    (a, b) = yakubun(a, b);
    if a == 1 {
        create_bunsu(n)
    } else {
        (a, b)
    }
}

// a以上b未満の乱数を取得
fn get_rand(a: i32, b :i32) -> i32 {
    let mut res: i32 = rand::thread_rng().gen_range(a, b-1);
    if res >= 0 { // 0は出ないようにする
        res += 1;
    }
    res
}
// 真分数・仮分数で約分
fn yakubun(mut a: i32, mut b: i32) -> (i32, i32) {
    let t: i32 = a.min(b);
    for i in (2..=t).rev() {
        if a % i == 0 && b % i == 0 {
            a /= i;
            b /= i;
        }
    }
    (a, b)
}