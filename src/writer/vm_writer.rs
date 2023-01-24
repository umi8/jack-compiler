use std::io::Write;

use anyhow::Result;

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

    pub fn write_return(written: &mut impl Write) -> Result<()> {
        writeln!(written, "return")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::writer::vm_writer::VmWriter;

    #[test]
    fn can_write_return() {
        let expected = "\
        return
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_return(&mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }
}
