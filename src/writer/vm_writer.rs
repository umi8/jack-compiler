use std::io::Write;

use anyhow::Result;

use crate::writer::command::Command;
use crate::writer::segment::Segment;

pub struct VmWriter {}

impl VmWriter {
    #[allow(dead_code)]
    pub fn write_push(segment: &Segment, index: usize, written: &mut impl Write) -> Result<()> {
        writeln!(written, "push {} {}", segment, index)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_pop(segment: &Segment, index: usize, written: &mut impl Write) -> Result<()> {
        writeln!(written, "pop {} {}", segment, index)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_arithmetic(command: &Command, written: &mut impl Write) -> Result<()> {
        writeln!(written, "{}", command)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_label(_label: &str) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn write_goto(_label: &str) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn write_if(_label: &str) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn write_call(name: &str, n_args: usize, written: &mut impl Write) -> Result<()> {
        writeln!(written, "call {} {}", name, n_args)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_function(name: &str, n_locals: usize, written: &mut impl Write) -> Result<()> {
        writeln!(written, "function {} {}", name, n_locals)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_return(written: &mut impl Write) -> Result<()> {
        writeln!(written, "return")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::writer::command::Command;
    use crate::writer::segment::Segment;
    use crate::writer::vm_writer::VmWriter;

    #[test]
    fn can_write_push() {
        let expected = "\
        push argument 0
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_push(&Segment::Argument, 0, &mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_write_pop() {
        let expected = "\
        pop this 1
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_pop(&Segment::This, 1, &mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_write_arithmetic() {
        let expected = "\
        add
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_arithmetic(&Command::Add, &mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_write_call() {
        let expected = "\
        call Math.multiply 2
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_call("Math.multiply", 2, &mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_write_function() {
        let expected = "\
        function Main.main 2
";
        let mut output = Vec::<u8>::new();
        VmWriter::write_function("Main.main", 2, &mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        assert_eq!(expected, actual)
    }

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
