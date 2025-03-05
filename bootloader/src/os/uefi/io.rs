use core::fmt::{self, Write};

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let st = super::system_table();

        for c in s.chars() {
            let _ = (st.ConsoleOut.OutputString)(st.ConsoleOut, [c as u16, 0].as_ptr());
            if c == '\n' {
                let _ = (st.ConsoleOut.OutputString)(st.ConsoleOut, ['\r' as u16, 0].as_ptr());
            }
        }

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}