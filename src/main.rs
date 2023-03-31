extern crate rand;

use rand::Rng;
use std::io::{stdout, stdin, Write, BufRead};
use std::fs::File;
use std::process::{Command};

fn main() {
    let all_step: i32 = 5;

    print!("0:約分  1:掛け算  2:割り算  :");
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();  // 標準入力から行を読み取る
    let selected_problem : usize = buffer.trim().parse().unwrap();  // 文字列を数値に変換する

    print!("問題数: ");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();  // 標準入力から行を読み取る
    let n : usize = buffer.trim().parse().unwrap();  // 文字列を数値に変換する
    progress(all_step, 0);

    // 問題・解答を生成してvecに入れる
    let mut v: Vec<(String, String)> = Vec::new();
    for _i in 0..n {
        let mut tmp: (String, String) = ("".to_string(), "".to_string());
        match selected_problem {
            0 => tmp = yakubun_item(),
            1 => tmp = kakezan_item(),
            2 => tmp = warizan_item(),
            _ => {},
        }
        v.push(tmp);
    }
    progress(all_step, 1);

    // 問題・解答を文字列化
    let mut ques: String = String::new();
    let mut ans: String = String::new();
    for i in 0..n {
        ques += &format!("\t\t\t{}\n", v[i].0);
        ans += &format!("\t\t\t{}\n", v[i].1);
    }
    progress(all_step, 2);

    // texファイルに書き込み
    write_tex("./main.tex".to_string(), ques, ans, n, selected_problem);
    progress(all_step, 3);

    //コンパイル
    shell(all_step);
}

// 約分問題1つを生成 帯分数だと数字が小さくなって難易度が落ちるので仮分数にしておく
fn yakubun_item() -> (String, String) {
    let a: i32;
    let b: i32;
    (a, b) = create_bunsu(14);
    
    let k = get_rand(2, 10);

    let res1: String = format!("\\item $\\displaystyle {}$", bunsu_text((a*k, b*k, 0)));
    let res2: String = format!("\\item $\\displaystyle {}$", bunsu_text((a, b, 0)));
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
    
    let res1: String = format!("\\item $\\displaystyle {} \\times {}$", bunsu_text(taibunsu(a, b)), bunsu_text(taibunsu(c, d)));
    let res2: String = format!("\\item $\\displaystyle {}$", bunsu_text(taibunsu(e, f)));
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
    
    let res1: String = format!("\\item $\\displaystyle {} \\div {}$", bunsu_text(taibunsu(a, b)), bunsu_text(taibunsu(c, d)));
    let res2: String = format!("\\item $\\displaystyle {}$", bunsu_text(taibunsu(e, f)));
    (res1, res2)
}

fn bunsu_text(t: (i32, i32, i32)) -> String {
    let mut res = String::new();
    if t.2 != 0 {
        res += &t.2.to_string();
        res += &" ";
    }
    if t.1 != 0 {
        res += &format!("\\frac{{{}}}{{{}}}", t.1, t.0);
    }
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

// texファイルに書き込み
fn write_tex(path: String, ques: String, ans: String, n: usize, selected_problem: usize) {
       let mut file = File::create(path)
           .expect("file not found.");
        writeln!(file, "\\documentclass[11pt,a4paper,dvipdfmx]{{jsarticle}}").expect("cannot write.");
        writeln!(file, "\\usepackage{{amsmath,amssymb, minijs, pxfonts, multicol, enumerate}}").expect("cannot write.");
        writeln!(file, "\\usepackage[top=25.4truemm,bottom=25.4truemm,left=19.05truemm,right=19.05truemm]{{geometry}}").expect("cannot write.");
        writeln!(file, "\\begin{{document}}").expect("cannot write.");

        
        match selected_problem {
            0 => writeln!(file, "\t\\subsection*{{問題 仮分数で約分しましょう．}}").expect("cannot write."),
            _ => writeln!(file, "\t\\subsection*{{問題}}").expect("cannot write."),
        }
        writeln!(file, "\t\\begin{{multicols*}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        writeln!(file, "\\setlength{{\\itemsep}}{{3mm}}").expect("cannot write.");
        write!(file, "{}", ques).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\end{{multicols*}}").expect("cannot write.");

        writeln!(file, "\t\\newpage").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{解答}}").expect("cannot write.");
        writeln!(file, "\t\\begin{{multicols*}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        writeln!(file, "\\setlength{{\\itemsep}}{{3mm}}").expect("cannot write.");
        write!(file, "{}", ans).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\subsection*{{正答率}}").expect("cannot write.");
        writeln!(file, "\t\\Huge\\hspace{{1cm}} /{}", n).expect("cannot write.");
        writeln!(file, "\t\\end{{multicols*}}").expect("cannot write.");

        writeln!(file, "\\end{{document}}").expect("cannot write.");
}

fn shell(all_step: i32) {
    let mut _output;
    _output = Command::new("platex")
        .arg("main.tex")
        .output()
        .expect("failed");
    progress(all_step, 4);
    _output = Command::new("dvipdfmx")
        .arg("main.dvi")
        .output()
        .expect("failed");
    progress(all_step, 5);
}

// k/n
fn progress(n: i32, k: i32) {
    print!("\x1b[2K");
    print!("\r");
    stdout().flush().unwrap();

    for _i in 0 .. k {
        print!("■");
    }
    for _i in k+1 ..= n {
        print!("□");
    }
    print!(" ");
    print!("{:.1}%", k as f64 / n as f64 * 100.0);
    stdout().flush().unwrap();
}