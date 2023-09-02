// 1115.
// https://leetcode.com/problems/print-foobar-alternately/
#[allow(dead_code)]
struct FooBar {
    n: usize,
}

#[allow(dead_code)]
impl FooBar {
    fn new(n: usize) -> FooBar {
        FooBar { n }
    }

    fn foo<F: Fn() -> ()>(&self, print_foo: F) {
        print_foo();
    }

    fn bar<F: Fn() -> ()>(&self, print_bar: F) {
        print_bar();
    }
}

#[cfg(test)]
mod benches {
    use std::{
        sync::{Arc, RwLock},
        thread::{self, JoinHandle},
        time::Duration,
    };

    use itertools::Itertools;
    use test::Bencher;

    use super::FooBar;

    #[derive(Default)]
    struct Printer {
        result: RwLock<String>,
    }

    impl Printer {
        fn new() -> Self {
            Default::default()
        }

        fn print_foo(&self) {
            self.result.write().unwrap().push_str("1");
        }

        fn print_bar(&self) {
            self.result.write().unwrap().push_str("2");
        }

        fn run(&self, foo: &FooBar, n: usize) {
            match n {
                1 => foo.foo(|| self.print_foo()),
                2 => foo.bar(|| self.print_bar()),
                _ => panic!("unexpected number"),
            }
        }

        fn result(&self) -> String {
            self.result.read().unwrap().clone()
        }
    }

    #[test]
    fn order() {
        let foo = Arc::new(FooBar::new(1));
        for data in [1, 2].into_iter().permutations(2) {
            let printer = Arc::new(Printer::new());
            for i in data {
                spawn(foo.clone(), printer.clone(), i, 1);
                thread::sleep(Duration::from_micros(10));
            }
            while printer.result().len() < 2 {
                thread::sleep(Duration::from_nanos(10));
            }
            assert_eq!("12", printer.result());
        }
    }
    const COUNT: usize = 10_000;

    fn spawn(foo: Arc<FooBar>, printer: Arc<Printer>, n: usize, count: usize) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..count {
                printer.run(&foo, n);
            }
        })
    }

    #[bench]
    fn bench_count(bencher: &mut Bencher) {
        let foo = Arc::new(FooBar::new(COUNT));
        {
            let foo = foo.clone();
            bencher.iter(move || {
                let printer = Arc::new(Printer::new());
                for i in 1..=2 {
                    let foo = foo.clone();
                    let printer = printer.clone();
                    spawn(foo, printer, i, COUNT);
                }

                while printer.result().len() < COUNT * 2 {
                    thread::sleep(Duration::from_nanos(10));
                }
                printer
                    .result()
                    .chars()
                    .enumerate()
                    .for_each(|(i, c)| assert_eq!((i % 2 + 1) as u32, c.to_digit(10).unwrap()));
            });
        }
    }
}
