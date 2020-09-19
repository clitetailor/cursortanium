#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

use regex::Regex;
use test::Bencher;

lazy_static! {
    static ref MATCH_STRING: &'static str = "Hi";
    static ref DOC: String = {
        let doc = "How do you do!";
        let mut chunks = vec![];
        for _i in 0..100 {
            chunks.push(doc.clone());
        }
        chunks.push(MATCH_STRING.clone());
        chunks.join("")
    };
    static ref FULL_STRING_RE: Regex =
        Regex::new(&MATCH_STRING).unwrap();
    static ref SUBSTRING_RE: Regex =
        Regex::new(&(r"\A".to_owned() + MATCH_STRING.clone()))
            .unwrap();
}

#[bench]
fn bench_regex_against_full_string(b: &mut Bencher) {
    let doc_len = DOC.len();

    b.iter(|| {
        for i in 0..doc_len {
            let mat = FULL_STRING_RE
                .find_at(&DOC, i)
                .filter(|mat| mat.start() == i);

            match mat {
                Some(_) => break,
                None => (),
            }
        }
    });
}

#[bench]
fn bench_regex_against_substring(b: &mut Bencher) {
    let doc_len = DOC.len();

    b.iter(|| {
        for i in 0..doc_len {
            let doc = &DOC[i..doc_len];

            let mat = SUBSTRING_RE.find(&doc);

            match mat {
                Some(_) => break,
                _ => (),
            }
        }
    });
}

#[bench]
fn bench_no_regex_match(b: &mut Bencher) {
    let doc_len = DOC.len();
    let match_string_len = MATCH_STRING.len();

    b.iter(|| {
        for i in 0..doc_len {
            if DOC[i..(i + match_string_len)].to_owned()
                == MATCH_STRING.to_owned()
            {
                break;
            };
        }
    });
}
