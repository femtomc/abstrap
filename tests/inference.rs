use abstraps::builder::ExtIRBuilder;
use abstraps::interp::{Communication, Interpreter, InterpreterError, Meta, Propagation};
use abstraps::ir::{AbstractInterpreter, Instruction, Operator};
use std::fmt;

// --------------- Smoke test --------------- //

// Fully specify the interpreter interface.

#[derive(Debug)]
pub enum Intrinsic0 {
    Fake,
}

impl fmt::Display for Intrinsic0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fake")
    }
}

#[derive(Debug)]
pub enum Attribute0 {
    Fake,
}

impl fmt::Display for Attribute0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fake")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lattice0 {
    Fake,
    Other(String),
}

#[derive(Debug)]
struct Global;
impl Communication<Meta<Lattice0>, Lattice0> for Global {
    fn ask(&self, msg: &Meta<Lattice0>) -> Option<Lattice0> {
        return None;
    }
}

impl Propagation<Intrinsic0, Attribute0, Lattice0, InterpreterError>
    for Interpreter<Intrinsic0, Attribute0, Lattice0, Global>
{
    fn propagate(
        &mut self,
        instr: &Instruction<Intrinsic0, Attribute0>,
    ) -> Result<Lattice0, InterpreterError> {
        return Ok(Lattice0::Fake);
    }
}

#[test]
fn infer_0() {
    let mut builder = ExtIRBuilder::<Intrinsic0, Attribute0>::default();
    let v = builder.push_arg();
    builder.build_instr(Operator::Intrinsic(Intrinsic0::Fake), vec![v], Vec::new());
    let args = builder.jump_blk(vec![v]);
    builder.build_instr(Operator::Intrinsic(Intrinsic0::Fake), args, Vec::new());
    let ir = builder.dump();
    println!("{}", ir);
    let m = Meta::new("".to_string(), vec![Lattice0::Fake]);
    let mut interp =
        Interpreter::<Intrinsic0, Attribute0, Lattice0, Global>::prepare(m, &ir).unwrap();
    interp.step(&ir);
    interp.step(&ir);
    interp.step(&ir);
    interp.step(&ir);
    println!("{:?}", interp);
    let analysis = interp.result();
    println!("{:?}", analysis);
}