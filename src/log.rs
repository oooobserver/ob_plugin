#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print!("{}", format_args!(concat!("\x1b[31m ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! success {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print!("{}", format_args!(concat!("\x1b[35m ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_color() {
        for i in 30..80 {
            let s = format!("\x1b[{}masdasdadsad\x1b[0m\n", i);
            println!("{}: {}", i, s);
        }
    }
}
