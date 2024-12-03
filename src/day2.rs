#[derive(Clone, Copy)]
struct JudgeState {
    last: Option<i64>,
    rising: Option<bool>,
}

#[inline(always)]
fn judge_number(p: i64, js: &mut JudgeState) -> bool {
    if let Some(last) = js.last {
        if last == p {
            return false;
        }
        let diff: i64 = last - p;
        if diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            // we are falling
            match js.rising {
                Some(true) => return false,
                Some(false) => {},
                None => js.rising = Some(false)
            }
        } else {
            // we are rising
            match js.rising {
                Some(false) => return false,
                Some(true) => {},
                None => js.rising = Some(true),
            }
        }
    }
    js.last = Some(p);
    return true;
}

#[inline(always)]
fn judge_line(l: &str) -> bool {
    let mut js = JudgeState{last: None, rising: None};
    for n in l.split(' ') {
        let p = n.parse().unwrap();
        if !judge_number(p, &mut js) {
            return false;
        }
    }
    return true;
}

#[aoc(day2, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let mut res = 0;
    for l in input.lines() {
        if judge_line(l) {
            res += 1;
        }
    }
    res
}

#[inline(always)]
fn judge_line_tolerate(l: &str) -> bool {
    let mut js = JudgeState{last: None, rising: None};
    let mut oldState = JudgeState{last: None, rising: None};
    let mut iter = l.split(' ').peekable();
    let mut skipped = false;
    loop {
        oldState = js;
        if let Some(p) = iter.peek() {
            let pp = p.parse().unwrap();
            if judge_number(pp, &mut js) {
                // nom
                iter.next();
            } else {
                // skip?
                if skipped {
                    return false;
                }
                js = oldState;
                iter.next();
                skipped = true;
            }
        } else {
            break;
        }
    }
    return true;
}

#[aoc(day2, part2, Chars)]
pub fn part2_chars(input: &str) -> i32 {
    let mut res = 0;
    'main: for l in input.lines() {
        if judge_line_tolerate(l) {
            res += 1;
        } else {
            let splits = l.split(' ');
            let nums: Vec<i64> = splits.map(|n| n.parse().unwrap()).collect();
            'brute: for i in 0..nums.len() {
                let mut js = JudgeState{last: None, rising: None};
                for j in 0..nums.len() {
                    if i == j { continue; }
                    if !judge_number(nums[j], &mut js) {
                        continue 'brute;
                    }
                }
                res += 1;
                continue 'main;
            }
        }
    }
    res
}
