use crate::{parser::parse, program::result_to_string};

pub fn run() {
    let input = include_str!("../input.txt");
    let (_input, mut prog) = parse(input).unwrap();

    let part1 = prog.run();
    println!("part1: {}", result_to_string(&part1));
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse, program::result_to_string};

    #[test]
    fn demo_input() {
        let input = include_str!("../demo.txt");
        let (input, mut prog) = parse(input).expect("valid parse");
        assert!(input.is_empty());
        let out = prog.run();
        let out = result_to_string(&out);
        assert_eq!(&out, "4,6,3,5,6,3,5,2,1,0");
    }
}
