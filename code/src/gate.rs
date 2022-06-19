pub fn nand(x: bool, y: bool) -> bool {
    !(x && y)
}

pub fn not(x: bool) -> bool {
    nand(x, x)
}

pub fn and(x: bool, y: bool) -> bool {
    not(nand(x, y))
}

pub fn or(x: bool, y: bool) -> bool {
    not(and(not(x), not(y)))
}

pub fn xor(x: bool, y: bool) -> bool {
    and(or(x, y), not(and(x, y)))
}

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
    or(and(x, !sel), and(y, sel))
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    (and(input, !sel), and(input, sel))
}
