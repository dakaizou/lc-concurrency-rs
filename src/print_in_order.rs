// 1114.
// https://leetcode.com/problems/print-in-order/

#[allow(dead_code)]
struct Foo;

#[allow(dead_code)]
impl Foo {
    fn new() -> Foo {
        Foo
    }

    fn first<F: Fn() -> ()>(&self, print_first: F) {
        print_first();
    }

    fn second<F: Fn() -> ()>(&self, print_second: F) {
        print_second();
    }

    fn third<F: Fn() -> ()>(&self, print_third: F) {
        print_third();
    }
}

#[cfg(test)]
mod benches {
    use std::{
        sync::{Arc, Mutex},
        thread::{self, JoinHandle},
        time::Duration,
    };

    use itertools::Itertools;
    use test::Bencher;

    use super::Foo;

    #[derive(Default)]
    struct Printer {
        result: Mutex<String>,
    }

    impl Printer {
        fn new() -> Self {
            Default::default()
        }

        fn print_first(&self) {
            self.result.lock().unwrap().push_str("1");
        }

        fn print_second(&self) {
            self.result.lock().unwrap().push_str("2");
        }

        fn print_third(&self) {
            self.result.lock().unwrap().push_str("3");
        }

        fn run(&self, foo: &Foo, n: usize) {
            match n {
                1 => foo.first(|| self.print_first()),
                2 => foo.second(|| self.print_second()),
                3 => foo.third(|| self.print_third()),
                _ => panic!("unexpected number"),
            }
        }

        fn result(&self) -> String {
            self.result.lock().unwrap().clone()
        }
    }

    #[test]
    fn order() {
        let foo = Arc::new(Foo::new());
        for data in [1, 2, 3].into_iter().permutations(3) {
            let printer = Arc::new(Printer::new());
            for i in data {
                spawn(foo.clone(), printer.clone(), i, 1);
                thread::sleep(Duration::from_micros(10));
            }
            while printer.result().len() < 3 {
                thread::sleep(Duration::from_nanos(10));
            }
            assert_eq!("123", printer.result());
        }
    }
    const COUNT: usize = 10_000;

    fn spawn(foo: Arc<Foo>, printer: Arc<Printer>, n: usize, count: usize) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..count {
                printer.run(&foo, n);
            }
        })
    }

    #[bench]
    fn bench_count(bencher: &mut Bencher) {
        let foo = Arc::new(Foo::new());
        {
            let foo = foo.clone();
            bencher.iter(move || {
                let printer = Arc::new(Printer::new());
                for i in 1..=3 {
                    let foo = foo.clone();
                    let printer = printer.clone();
                    spawn(foo, printer, i, COUNT);
                }

                while printer.result().len() < COUNT * 3 {
                    thread::sleep(Duration::from_nanos(10));
                }
                printer
                    .result()
                    .chars()
                    .enumerate()
                    .for_each(|(i, c)| assert_eq!((i % 3 + 1) as u32, c.to_digit(10).unwrap()));
            });
        }
    }
}
