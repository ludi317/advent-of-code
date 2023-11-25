use nom::{bytes::complete::{tag}, character::complete::{char, digit1, line_ending, space1}};
use nom::{combinator::{map, map_res, recognize, value}, multi::separated_list1};
use nom::{branch::alt, sequence::{pair, preceded}, IResult, Parser};

fn offset(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(
                alt((char('-'), char('+'))).and(digit1)
        ),
        |s: &str| s.parse::<isize>(),
    )(input)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    A,
    B,
}

fn register(input: &str) -> IResult<&str, Register> {
    alt((
        value(Register::A, tag("a")),
        value(Register::B, tag("b")),
    ))(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("hlf"), preceded(space1, register)), Instruction::Hlf),
        map(preceded(tag("tpl"), preceded(space1, register)), Instruction::Tpl),
        map(preceded(tag("inc"), preceded(space1, register)), Instruction::Inc),
        map(preceded(tag("jmp"), preceded(space1, offset)), Instruction::Jmp),
        map(
            pair(preceded(tag("jie"), preceded(space1, register)), preceded(tag(", "), offset)),
            |(reg, ofs)| Instruction::Jie(reg, ofs),
        ),
        map(
            pair(preceded(tag("jio"), preceded(space1, register)), preceded(tag(", "), offset)),
            |(reg, ofs)| Instruction::Jio(reg, ofs),
        ),
    ))(input)
}

fn program(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(
            line_ending,
            instruction

    )(input)
}


#[derive(Debug, PartialEq, Eq)]
pub struct Vm {
    prog: Vec<Instruction>,
    a: usize,
    b: usize,
    ip: usize,
}

impl<'a> From<&'a str> for Vm {
    fn from(s: &str) -> Vm {
        Vm {
            prog: program(s).unwrap().1,
            a: 0,
            b: 0,
            ip: 0,
        }
    }
}

impl Vm {
    fn jump(&mut self, offset: isize) {
        if offset < 0 {
            self.ip -= (-offset) as usize;
        } else {
            self.ip += offset as usize;
        }
    }

    fn done(&self) -> bool {
        self.ip >= self.prog.len()
    }

    fn step(&mut self) {
        match self.prog[self.ip] {
            Instruction::Hlf(Register::A) => { self.a /= 2; self.jump(1); },
            Instruction::Hlf(Register::B) => { self.b /= 2; self.jump(1); },
            Instruction::Tpl(Register::A) => { self.a *= 3; self.jump(1); },
            Instruction::Tpl(Register::B) => { self.b *= 3; self.jump(1); },
            Instruction::Inc(Register::A) => { self.a += 1; self.jump(1); },
            Instruction::Inc(Register::B) => { self.b += 1; self.jump(1); },
            Instruction::Jmp(ofs) => self.jump(ofs),
            Instruction::Jie(Register::A, ofs) => if self.a % 2 == 0 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jie(Register::B, ofs) => if self.b % 2 == 0 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jio(Register::A, ofs) => if self.a == 1 { self.jump(ofs) } else { self.jump(1) },
            Instruction::Jio(Register::B, ofs) => if self.b == 1 { self.jump(ofs) } else { self.jump(1) },
        }
    }

    fn run(&mut self) {
        while !self.done() {
            self.step();
        }
    }
}

fn main() {

    let mut vm = Vm::from(include_str!("../input/day23.txt"));
    vm.run();
    println!("Value of register B after running program: {}", vm.b);
    let mut vm = Vm::from(include_str!("../input/day23.txt"));
    vm.a = 1;
    vm.run();
    println!("Value of register B after running program if register A starts as 1: {}", vm.b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(program("inc a\njio a, +2\ntpl a\ninc a"), Ok(("", vec![
            Instruction::Inc(Register::A),
            Instruction::Jio(Register::A, 2),
            Instruction::Tpl(Register::A),
            Instruction::Inc(Register::A),
        ])));
    }

    #[test]
    fn parsing_complete() {
        let vm = Vm::from(include_str!("../input/day23.txt"));
        assert_eq!(vm.prog.len(), 47);
    }

    #[test]
    fn stepping() {
        let mut vm = Vm::from("inc a\njio a, +2\ntpl a\ninc a");
        vm.step();
        assert_eq!(vm.a, 1);
        vm.step();
        assert_eq!(vm.ip, 3);
        vm.step();
        assert_eq!(vm.a, 2);
        assert!(vm.done());
    }

    #[test]
    fn running() {
        let mut vm = Vm::from("inc a\njio a, +2\ntpl a\ninc a");
        vm.run();
        assert_eq!(vm.a, 2);
        assert!(vm.done());
    }
}