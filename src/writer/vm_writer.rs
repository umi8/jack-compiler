use crate::writer::command::Command;
use crate::writer::segment::Segment;

pub struct VmWriter {}

impl VmWriter {
    pub fn write_push(segment: &Segment, index: usize) {
        todo!()
    }

    pub fn write_pop(segment: &Segment, index: usize) {
        todo!()
    }

    pub fn write_arithmetic(command: &Command) {
        todo!()
    }

    pub fn write_label(label: &str) {
        todo!()
    }

    pub fn write_goto(label: &str) {
        todo!()
    }

    pub fn write_if(label: &str) {
        todo!()
    }

    pub fn write_call(name: &str, n_args: usize) {
        todo!()
    }

    pub fn write_function(name: &str, n_locals: usize) {
        todo!()
    }

    pub fn write_return() {
        todo!()
    }
}
