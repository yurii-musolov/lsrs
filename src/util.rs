use regex::Regex;

pub fn by(re: Option<Regex>, invert: bool) -> impl Fn(&str) -> bool {
    move |s| -> bool {
        match &re {
            Some(re) => {
                if invert {
                    !re.is_match(s)
                } else {
                    re.is_match(s)
                }
            }
            None => true,
        }
    }
}

pub fn temporary(ignore: bool) -> impl Fn(&str) -> bool {
    move |s| if ignore { !s.ends_with("~") } else { true }
}

pub fn every<'a>(fs: &'a Vec<&'a dyn Fn(&str) -> bool>) -> impl Fn(&str) -> bool + use<'a> {
    move |s| {
        let mut res = false;

        for f in fs {
            res = f(s);
            if !res {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_work() {
        let cases = vec![("qwerty.txt", true), ("abcxyz.txt", false)];
        let regex = Some(Regex::new(&"qwerty").unwrap());
        let check = by(regex, false);

        cases
            .iter()
            .for_each(|(s, expected)| assert_eq!(check(s), *expected));
    }

    #[test]
    fn temporary_work() {
        let cases = vec![
            (false, vec![("file.txt~", true), ("file.txt", true)]),
            (true, vec![("file.txt~", false), ("file.txt", true)]),
        ];

        cases.iter().for_each(|(ignore, cases)| {
            let check = temporary(*ignore);
            cases
                .iter()
                .for_each(|(s, expected)| assert_eq!(check(s), *expected));
        });
    }

    #[test]
    fn every_work() {
        let cases: Vec<(Vec<&dyn Fn(&str) -> bool>, bool)> = vec![
            (vec![&|_| true, &|_| false], false),
            (vec![&|_| true, &|_| true], true),
        ];

        cases.iter().for_each(|(fs, expected)| {
            let check = every(fs);
            assert_eq!(check(""), *expected);
        });
    }
}
