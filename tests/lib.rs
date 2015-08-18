#![feature(plugin)]
#![plugin(power_of_ten)]

pub fn unbounded() {
    for _var in (1..) { };
    for _var in (1..).map(|x| x + 5) { };
}

pub fn bounded() {
    for _var in (1..10) { };
    for _var in (1..10).map(|x| x + 5) { };
    for _var in (1..).take(5) { };
    for _var in (1..).take(5).map(|x| x + 5) { };
    for _var in (1..).map(|x| x + 5).take(5) { };
}
