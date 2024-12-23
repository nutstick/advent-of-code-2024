use std::error::Error;

use z3::{
    ast::{self, Ast},
    Config, Context, Optimize, SatResult,
};

use crate::errors::ParseInputError;

#[derive(Debug, PartialEq)]
enum State {
    Running,
    Halted,
}

struct Program {
    pub program: Vec<u32>,
    pub instruction_pointer: usize,
    pub register_a: u32,
    pub register_b: u32,
    pub register_c: u32,
    pub state: State,
    pub output: Vec<u32>,
}

impl Program {
    pub fn from_input(input: &str) -> Result<Self, ParseInputError> {
        let (registers_input, program_input) = input.split_once("\n\n").ok_or(ParseInputError)?;
        let mut lines = registers_input.lines();
        let register_a = Self::parse_register("A", lines.next())?;
        let register_b = Self::parse_register("B", lines.next())?;
        let register_c = Self::parse_register("C", lines.next())?;
        let program = program_input
            .replace("Program: ", "")
            .split(",")
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .or(Err(ParseInputError))?;
        Ok(Self {
            program,
            instruction_pointer: 0,
            register_a,
            register_b,
            register_c,
            state: State::Running,
            output: Vec::new(),
        })
    }

    fn parse_register(register_name: &str, line: Option<&str>) -> Result<u32, ParseInputError> {
        match line {
            Some(line) => line
                .replace(&"Register *: ".replace("*", register_name), "")
                .parse::<u32>()
                .map_err(|_| ParseInputError),
            None => Err(ParseInputError),
        }
    }
}

impl Program {
    pub fn tick(&mut self) {
        if self.state == State::Halted {
            return;
        }

        match self.program.get(self.instruction_pointer) {
            Some(opcode) => {
                let operator = self.program[self.instruction_pointer + 1];
                match opcode {
                    0 => {
                        self.register_a >>= self.combo_operand(operator);
                        self.instruction_pointer += 2;
                    }
                    1 => {
                        self.register_b ^= operator;
                        self.instruction_pointer += 2;
                    }
                    2 => {
                        self.register_b = self.combo_operand(operator) % 8;
                        self.instruction_pointer += 2;
                    }
                    3 => {
                        if self.register_a != 0 {
                            self.instruction_pointer = operator as usize;
                        } else {
                            self.instruction_pointer += 2;
                        }
                    }
                    4 => {
                        self.register_b ^= self.register_c;
                        self.instruction_pointer += 2;
                    }
                    5 => {
                        self.output.push(self.combo_operand(operator) & 7);
                        self.instruction_pointer += 2;
                    }
                    6 => {
                        self.register_b = self.register_a >> self.combo_operand(operator);
                        self.instruction_pointer += 2;
                    }
                    7 => {
                        self.register_c = self.register_a >> self.combo_operand(operator);
                        self.instruction_pointer += 2;
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                self.state = State::Halted;
                return;
            }
        }
    }

    fn combo_operand(&self, operator: u32) -> u32 {
        match operator {
            0..=3 => operator,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut program = Program::from_input(input)?;

    loop {
        if program.state == State::Halted {
            break;
        }
        program.tick();
    }

    // Vec<u32> to string with comma
    Ok(program
        .output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

#[derive(Debug)]
struct ProgramZ3<'ctx> {
    pub program: Vec<u32>,
    pub instruction_pointer: usize,
    pub register_a: ast::BV<'ctx>,
    pub register_b: ast::BV<'ctx>,
    pub register_c: ast::BV<'ctx>,
    pub state: State,

    // z3 part
    assumptions: Vec<ast::Bool<'ctx>>,
    ctx: &'ctx Context,
    n_out_commands: usize,
}

impl<'ctx> ProgramZ3<'ctx> {
    pub fn from_input(input: &str, ctx: &'ctx Context) -> Result<Self, ParseInputError> {
        let (registers_input, program_input) = input.split_once("\n\n").ok_or(ParseInputError)?;
        let mut lines = registers_input.lines();
        lines.next();
        let register_b = Self::parse_register("B", lines.next())?;
        let register_c = Self::parse_register("C", lines.next())?;
        let program = program_input
            .replace("Program: ", "")
            .split(",")
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .or(Err(ParseInputError))?;

        Ok(Self {
            program,
            instruction_pointer: 0,
            register_a: ast::BV::new_const(&ctx, "a", 64),
            register_b: ast::BV::from_i64(&ctx, register_b as i64, 64),
            register_c: ast::BV::from_i64(&ctx, register_c as i64, 64),
            state: State::Running,
            assumptions: vec![],
            ctx: &ctx,
            n_out_commands: 0,
        })
    }

    fn parse_register(register_name: &str, line: Option<&str>) -> Result<u32, ParseInputError> {
        match line {
            Some(line) => line
                .replace(&"Register *: ".replace("*", register_name), "")
                .parse::<u32>()
                .map_err(|_| ParseInputError),
            None => Err(ParseInputError),
        }
    }
}

impl<'ctx> ProgramZ3<'ctx> {
    fn tick(&mut self) {
        if self.state == State::Halted {
            return;
        }

        match self.program.get(self.instruction_pointer) {
            Some(opcode) => {
                let operator = self.program[self.instruction_pointer + 1];
                match opcode {
                    0 => {
                        self.register_a = self.register_a.bvlshr(&self.combo_operand(operator));
                        self.instruction_pointer += 2;
                    }
                    1 => {
                        self.register_b = self.register_b.bvxor(&ast::BV::from_i64(
                            &self.ctx,
                            operator as i64,
                            64,
                        ));
                        self.instruction_pointer += 2;
                    }
                    2 => {
                        self.register_b = self
                            .combo_operand(operator)
                            .bvand(&ast::BV::from_i64(&self.ctx, 7, 64));
                        self.instruction_pointer += 2;
                    }
                    3 => {
                        if self.n_out_commands < self.program.len() {
                            let constraint = self
                                .register_a
                                ._eq(&ast::BV::from_i64(&self.ctx, 0, 64))
                                .not();
                            self.assumptions.push(constraint);
                            self.instruction_pointer = operator as usize;
                        } else {
                            let constraint =
                                self.register_a._eq(&ast::BV::from_i64(&self.ctx, 0, 64));
                            self.assumptions.push(constraint);
                            self.instruction_pointer += 2;
                        }
                    }
                    4 => {
                        self.register_b = self.register_b.bvxor(&self.register_c);
                        self.instruction_pointer += 2;
                    }
                    5 => {
                        let expected = self.program[self.n_out_commands];
                        let constraint = self
                            .combo_operand(operator)
                            .bvand(&ast::BV::from_i64(&self.ctx, 7, 64))
                            ._eq(&ast::BV::from_i64(&self.ctx, expected as i64, 64));

                        self.assumptions.push(constraint);
                        self.instruction_pointer += 2;
                        self.n_out_commands += 1;
                    }
                    6 => {
                        self.register_b = self.register_a.bvlshr(&self.combo_operand(operator));
                        self.instruction_pointer += 2;
                    }
                    7 => {
                        self.register_c = self.register_a.bvlshr(&self.combo_operand(operator));
                        self.instruction_pointer += 2;
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                self.state = State::Halted;
                return;
            }
        }
    }

    fn combo_operand(&self, operator: u32) -> ast::BV<'ctx> {
        match operator {
            0..=3 => ast::BV::from_i64(&self.ctx, operator as i64, 64),
            4 => self.register_a.clone(),
            5 => self.register_b.clone(),
            6 => self.register_c.clone(),
            _ => unreachable!(),
        }
    }
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut program = ProgramZ3::from_input(input, &ctx)?;
    let opt = Optimize::new(&ctx);
    let register_a = program.register_a.clone();
    opt.minimize(&register_a);

    loop {
        if program.state == State::Halted {
            break;
        }
        program.tick();
    }

    if let SatResult::Sat = opt.check(&program.assumptions) {
        let model = opt.get_model().unwrap();
        let a_val = model.eval(&register_a, false).unwrap();
        return Ok(a_val.as_i64().unwrap().to_string());
    }

    Err("No solution found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const SAMPLE_INPUT_2: &str = "\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    const SAMPLE_INPUT_3: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part1(SAMPLE_INPUT_1).unwrap(), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part1(SAMPLE_INPUT_2).unwrap(), "0,1,2");
        assert_eq!(part1(SAMPLE_INPUT_3).unwrap(), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(part2(SAMPLE_INPUT_3).unwrap(), "117440");
    }
}
