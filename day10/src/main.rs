use itertools::Itertools;
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
struct CRT {
    display_row: String,
    cols: usize,
}
#[derive(Clone, Copy)]
enum Pixel {
    Lit,
    Dark,
}

impl CRT {
    fn new(cols: usize) -> CRT {
        CRT {
            display_row: String::with_capacity(6 * cols),
            cols,
        }
    }
    fn draw(&mut self, pixel: Pixel) {
        self.display_row.push(match pixel {
            Pixel::Lit => '█',
            Pixel::Lit => '#',
            Pixel::Dark => '.',
        });
    }
    fn current_position_col(&self) -> usize {
        self.display_row.len() % self.cols
    }
    fn next_pixel_position(&self) -> usize {
        self.display_row.chars().count()
    }
}

impl Processor {
    fn perform_cycle(&mut self, crt: &mut CRT) {
        self.cycle_count += 1;

        let sprite_position = self.x_register % crt.cols as i64;
        let draw_pixel_pos = crt.next_pixel_position() % crt.cols;
        let difference = sprite_position - draw_pixel_pos as i64;

        dbg!(sprite_position, draw_pixel_pos, difference);
        crt.draw(match difference {
            -1..=1 => Pixel::Lit,
            _ => Pixel::Dark,
        });
    }
    fn examine_cycle(&self) -> Option<ProcessorState> {
        if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 {
            return Some(ProcessorState {
                cycle: self.cycle_count,
                x_register: self.x_register,
            });
        }
        None
    }
    fn process_instruction(&mut self, ins: Instruction, crt: &mut CRT) -> Option<ProcessorState> {
        match ins {
            Instruction::Noop => {
                self.perform_cycle(crt);
                self.examine_cycle()
            }
            Instruction::Addx(x) => {
                self.perform_cycle(crt);
                let mut out = self.examine_cycle();
                self.perform_cycle(crt);
                if out.is_none() {
                    out = self.examine_cycle();
                }

                self.x_register += x;
                out
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
    const CRT_WIDTH: usize = 40;

    let input = include_str!("../input.txt");

    let instructions = input
        .lines()
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1);

    let mut processor = Processor::new();
    let mut crt = CRT::new(CRT_WIDTH);

    // for ins in instructions {
    //     if let Some(state) = processor.process_instruction(ins, &mut crt) {
    //         println!(
    //             "{}: {} {:?}",
    //             state.cycle,
    //             state.x_register * state.cycle as i64,
    //             ins
    //         );
    //     }
    // }
    let answer_p1 = instructions
        .filter_map(|i| processor.process_instruction(i, &mut crt))
        .map(|ps| ps.cycle as i64 * ps.x_register)
        .sum::<i64>();

    for chunk in &crt.display_row.chars().chunks(CRT_WIDTH) {
        println!("{}", chunk.collect::<String>());
    }

    println!("answer p1: {answer_p1}");
}
