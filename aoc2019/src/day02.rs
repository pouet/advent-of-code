use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Vm {
    mem: Vec<usize>,
    ip: usize,
}

impl Vm {
    fn from(s: &str) -> Vm {
        let mem = s
            .split(',')
            .flat_map(str::parse)
            .collect_vec();

        Vm {
            mem,
            ip: 0
        }
    }

    fn increment_ip(&mut self, n: usize) {
        self.ip += n;
    }

    fn pos(&self, n: usize) -> usize {
        self.mem[self.ip + n]
    }

    fn value(&self, n: usize) -> usize {
        self.mem[self.pos(n)]
    }

    fn add(&mut self) {
        let n1 = self.value(1);
        let n2 = self.value(2);
        let pos = self.pos(3);

        self.mem[pos] = n1 + n2;
        self.increment_ip(4);
    }

    fn mul(&mut self) {
        let n1 = self.value(1);
        let n2 = self.value(2);
        let pos = self.pos(3);

        self.mem[pos] = n1 * n2;
        self.increment_ip(4);
    }

    fn is_finished(&self) -> bool {
        self.mem[self.ip] == 99
    }

    fn execute_next(&mut self) {
        match self.mem[self.ip] {
            1 => self.add(),
            2 => self.mul(),
            _ => ()
        }
    }

    fn run(&mut self) {
        while !self.is_finished() {
            self.execute_next();
        }
    }
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vm {
    Vm::from(input)
}

#[aoc(day2, part1)]
pub fn solve_part1(vm: &Vm) -> usize {
    let mut vm = vm.clone();

    vm.mem[1] = 12;
    vm.mem[2] = 2;
    vm.run();

    vm.mem[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(vm: &Vm) -> Option<usize> {
    for (x, y) in iproduct!(0..99, 0..99) {
        let mut vm = vm.clone();

        vm.mem[1] = x;
        vm.mem[2] = y;
        vm.run();

        if vm.mem[0] == 19690720 {
            return Some(vm.mem[1] * 100 + vm.mem[2]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "1,9,10,3,2,3,11,0,99,30,40,50"
    }

    #[test]
    fn test_gen() {
        println!("{:?}", gen(get_input()));
    }

    #[test]
    fn test_part1() {
        let mut vm = gen("1,0,0,0,99");
        vm.run();
        println!("{:?}", vm);

        let mut vm = gen("2,3,0,3,99");
        vm.run();
        println!("{:?}", vm);

        let mut vm = gen("2,4,4,5,99,0");
        vm.run();
        println!("{:?}", vm);

        let mut vm = gen("1,1,1,4,99,5,6,0,99");
        vm.run();
        println!("{:?}", vm);
    }
}