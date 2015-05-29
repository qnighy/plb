
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn sd_genmat() -> (Vec<Vec<usize>>, Vec<[usize; 4]>) {
    let mut mr : Vec<Vec<usize>> = Vec::with_capacity(324);
    let mut mc : Vec<[usize; 4]> = Vec::with_capacity(729);
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..9 {
                mc.push([ 9 * i + j, (i/3*3 + j/3) * 9 + k + 81, 9 * i + k + 162, 9 * j + k + 243 ]);
            }
        }
    }
    for _ in 0..324 { mr.push(Vec::new()); }
    for (r, mcr) in mc.iter().enumerate() {
        for mcri in mcr {
            mr[*mcri].push(r);
        }
    }
    (mr, mc)
}

fn sd_update(
    mr : &Vec<Vec<usize>>,
    mc : &Vec<[usize; 4]>,
    sr : &mut Vec<isize>,
    sc : &mut Vec<isize>, r : usize, v : isize) {

    for c2 in 0..4 {
        let c = mc[r][c2];
        sc[c] += v;
        for p in &mr[c] {
            sr[*p] += v
        }
    }
}
fn sd_solve(
    mr : &Vec<Vec<usize>>,
    mc : &Vec<[usize; 4]>,
    s : &str) -> Vec<Vec<usize>> {
    let mut ret : Vec<Vec<usize>> = Vec::new();
    let mut sr : Vec<isize> = Vec::with_capacity(729);
    for _ in 0..729 { sr.push(0); }
    let mut sc : Vec<isize> = Vec::with_capacity(324);
    for _ in 0..324 { sc.push(0); }
    let mut hints = 0;
    let s_chars : Vec<char> = s.chars().collect();
    for i in 0..81 {
        let a =
            if '1' <= s_chars[i] && s_chars[i] <= '9' {
                (s_chars[i] as isize) - ('1' as isize)
            } else { -1 };
        if a >= 0 {
            sd_update(mr, mc, &mut sr, &mut sc, i * 9 + (a as usize), 1);
            hints += 1;
        }
    }
    let mut cr : Vec<isize> = Vec::with_capacity(81);
    for _ in 0..81 { cr.push(-1); }
    let mut cc : Vec<isize> = Vec::with_capacity(81);
    for _ in 0..81 { cc.push(-1); }
    let mut i : isize = 0;
    let mut c0 = 0;
    let mut dir : isize = 1;
    loop {
        while i >= 0 && i < 81 - hints {
            let ii = i as usize;
            if dir == 1 {
                let mut min = 10;
                for j in 0..324 {
                    let c : usize =
                        if j + c0 < 324 {
                            j + c0
                        } else {
                            j + c0 - 324
                        };
                    if sc[c] != 0 {
                        continue;
                    }
                    let mut n = 0;
                    for p in &mr[c] {
                        if sr[*p] == 0 {
                            n += 1;
                        }
                    }
                    if n < min {
                        min = n;
                        cc[ii] = c as isize;
                        c0 = c + 1;
                    }
                    if n <= 1 {
                        break;
                    }
                }
                if min == 0 || min == 10 {
                    cr[ii] = -1;
                    dir = -1;
                    i -= 1;
                }
            }
            let ii = i as usize;

            let c = cc[ii] as usize;
            if dir == -1 && cr[ii] >= 0 {
                sd_update(mr, mc, &mut sr, &mut sc, mr[c][cr[ii] as usize], -1);
            }
            let mut r2m : usize = 9;
            for r2 in ((cr[ii]+1) as usize)..9 {
                if sr[mr[c][r2]] == 0 {
                    r2m = r2;
                    break;
                }
            }
            if r2m < 9 {
                sd_update(mr, mc, &mut sr, &mut sc, mr[c][r2m], 1);
                cr[ii] = r2m as isize;
                dir = 1;
                i += 1;
            } else {
                cr[ii] = -1;
                dir = -1;
                i -= 1;
            }
        }
        if i < 0 {
            break;
        }
        let mut o : Vec<usize> = Vec::new();
        for j in 0..81 {
            if '1' <= s_chars[j] && s_chars[j] <= '9' {
                o.push((s_chars[j] as usize) - ('0' as usize));
            } else {
                o.push(0);
            }
        }
        for j in 0..(i as usize) {
            let r = mr[cc[j] as usize][cr[j] as usize];
            o[r/9] = r % 9 + 1;
        }
        ret.push(o);
        i = i - 1;
        dir = -1;
    }
    return ret;
}

fn main() {
    let stdin = io::stdin();

    let (mr, mc) = sd_genmat();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.len() >= 81 {
            let ret = sd_solve(&mr, &mc, &line);
            for s in ret {
                for c in s {
                    print!("{}", c);
                }
                println!("");
            }
            println!("");
        }
    }
}
