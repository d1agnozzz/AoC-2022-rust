use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    sequence::preceded,
    Finish, IResult,
};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Instruction> {
        alt((
            value(Instruction::Noop, tag("noop")),
            Instruction::parse_addx,
        ))(i)
    }
    fn parse_addx(i: &str) -> IResult<&str, Self> {
        map(
            preceded(tag("addx "), nom::character::complete::i64),
            Instruction::Addx,
        )(i)
    }
}

struct Processor {
    cycle_count: usize,
    x_register: i64,
}
struct ProcessorState {
    cycle: usize,
    x_register: i64,
}

impl Processor {
    fn perform_cycle(&mut self) -> Option<ProcessorState> {
        self.cycle_count += 1;
        if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 {
            return Some(ProcessorState {
                cycle: self.cycle_count,
                x_register: self.x_register,
            });
        }
        None
    }
    fn process_instruction(&mut self, ins: Instruction) -> Option<ProcessorState> {
        match ins {
            Instruction::Noop => self.perform_cycle(),
            Instruction::Addx(x) => {
                let sign_strength_c1 = self.perform_cycle();
                let mut sign_strength_c2 = self.perform_cycle();
                if sign_strength_c2.is_none() {
                    sign_strength_c2 = sign_strength_c1;
                }

                self.x_register += x;
                sign_strength_c2
            }
        }
    }
    fn new() -> Processor {
        Processor {
            cycle_count: 0,
            x_register: 1,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let instructions = input
        .lines()
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1);

    let mut processor = Processor::new();

    // for ins in instructions.it {
    //     if let Some(state) = processor.process_instruction(ins) {
    //         println!(
    //             "{}: {} {:?}",
    //             state.cycle,
    //             state.x_register * state.cycle as i64,
    //             ins
    //         );
    //     }
    // }
    let answer_p1 = instructions
        .filter_map(|i| processor.process_instruction(i))
        .map(|ps| ps.cycle as i64 * ps.x_register)
        .sum::<i64>();

    println!("answer p1: {answer_p1}");
}
