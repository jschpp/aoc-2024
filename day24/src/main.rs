use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending, space1},
    combinator::value,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_input, mut system) = parse(input).unwrap();
    let part1_result = system.simulate();
    println!("part1: {part1_result}");
}

#[derive(Debug)]
struct BinarySystem {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl BinarySystem {
    fn simulate(&mut self) -> usize {
        let mut outputs = self
            .gates
            .iter()
            .flat_map(|g| [(&g.lhs, None), (&g.rhs, None), (&g.out, None)])
            .collect::<HashMap<_, Option<bool>>>();
        for (k, v) in self.inputs.iter() {
            outputs
                .entry(k)
                .and_modify(|e| *e = Some(*v))
                .or_insert(Some(*v));
        }
        while outputs.iter().any(|(_k, v)| v.is_none()) {
            for gate in self.gates.iter() {
                if outputs.get(&gate.lhs).unwrap().is_none()
                    || outputs.get(&gate.rhs).unwrap().is_none()
                {
                    continue;
                }
                let lhs = outputs.get(&gate.lhs).unwrap().unwrap();
                let rhs = outputs.get(&gate.rhs).unwrap().unwrap();
                match gate.typ {
                    GateType::And => outputs
                        .entry(&gate.out)
                        .and_modify(|e| *e = Some(lhs & rhs)),
                    GateType::Or => outputs
                        .entry(&gate.out)
                        .and_modify(|e| *e = Some(lhs | rhs)),
                    GateType::Xor => outputs
                        .entry(&gate.out)
                        .and_modify(|e| *e = Some(lhs ^ rhs)),
                };
            }
        }
        let mut z = outputs
            .keys()
            .filter(|k| k.starts_with("z"))
            .collect::<Vec<_>>();
        z.sort();
        let mut result = 0_usize;
        for (pow, k) in z.into_iter().enumerate() {
            let bit = match outputs.get(k).unwrap().unwrap() {
                true => 1_usize,
                false => 0_usize,
            };
            result += bit << pow;
        }
        result
    }
}

#[derive(Debug)]
struct Gate {
    typ: GateType,
    lhs: String,
    rhs: String,
    out: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

fn parse(input: &str) -> IResult<&str, BinarySystem> {
    let (input, in_val) = parse_input_values(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, gates) = separated_list1(line_ending, parse_gate)(input)?;
    Ok((
        input,
        BinarySystem {
            inputs: in_val,
            gates,
        },
    ))
}

fn parse_input_values(input: &str) -> IResult<&str, HashMap<String, bool>> {
    separated_list1(
        line_ending,
        separated_pair(alphanumeric1, tag(": "), digit1.map(|d: &str| d == "1")),
    )
    .map(|x| {
        x.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, bool>>()
    })
    .parse(input)
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, lhs) = terminated(alphanumeric1, space1)(input)?;
    let (input, typ) = terminated(
        alt((
            value(GateType::And, tag("AND")),
            value(GateType::Or, tag("OR")),
            value(GateType::Xor, tag("XOR")),
        )),
        space1,
    )(input)?;
    let (input, rhs) = terminated(alphanumeric1, space1)(input)?;
    let (input, _) = tag("-> ")(input)?;
    let (input, out) = alphanumeric1(input)?;
    Ok((
        input,
        Gate {
            typ,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            out: out.to_string(),
        },
    ))
}
