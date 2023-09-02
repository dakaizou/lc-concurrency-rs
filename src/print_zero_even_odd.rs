// 1116.
// https://leetcode.com/problems/print-zero-even-odd/

#[allow(dead_code)]
struct ZeroEvenOdd {
    n: usize,
}

#[allow(dead_code, unused)]
impl ZeroEvenOdd {
    fn new(n: usize) -> Self {
        ZeroEvenOdd { n }
    }

    fn zero<F: Fn(usize) -> ()>(&self, print_number: F) {
        todo!()
    }

    fn even<F: Fn(usize) -> ()>(&self, print_number: F) {
        todo!()
    }

    fn odd<F: Fn(usize) -> ()>(&self, print_number: F) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread::{self, JoinHandle},
        time::Duration,
    };

    use itertools::Itertools;

    use super::ZeroEvenOdd;

    #[derive(Default)]
    struct Printer {
        result: Mutex<String>,
    }

    impl Printer {
        fn new() -> Self {
            Default::default()
        }

        fn print_number(&self, n: usize) {
            self.result
                .lock()
                .unwrap()
                .push(char::from_digit(n as u32, 10).unwrap());
        }

        fn run(&self, foo: &ZeroEvenOdd, n: usize) {
            match n {
                0 => foo.zero(|n| self.print_number(n)),
                1 => foo.odd(|n| self.print_number(n)),
                2 => foo.even(|n| self.print_number(n)),
                _ => panic!("unexpected number"),
            }
        }

        fn result(&self) -> String {
            self.result.lock().unwrap().clone()
        }
    }

    #[test]
    fn order() {
        let foo = Arc::new(ZeroEvenOdd::new(2));
        for data in [0, 1, 2].into_iter().permutations(3) {
            let printer = Arc::new(Printer::new());
            for i in data {
                spawn(foo.clone(), printer.clone(), i);
                thread::sleep(Duration::from_micros(10));
            }
            while printer.result().len() < 4 {
                thread::sleep(Duration::from_nanos(10));
            }
            assert_eq!("0102", printer.result());
        }
    }

    fn spawn(foo: Arc<ZeroEvenOdd>, printer: Arc<Printer>, n: usize) -> JoinHandle<()> {
        thread::spawn(move || {
            printer.run(&foo, n);
        })
    }
}
