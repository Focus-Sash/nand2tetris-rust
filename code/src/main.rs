mod gate;

fn main() {}

#[cfg(test)]
mod test {
    use crate::gate::*;
    #[test]
    fn nand_test() {
        assert_eq!(nand(true, true), false);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(false, false), true);
    }

    #[test]
    fn not_test() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
    }

    #[test]
    fn and_test() {
        assert_eq!(and(true, true), true);
        assert_eq!(and(true, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(false, false), false);
    }

    #[test]
    fn or_test() {
        assert_eq!(or(true, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(false, true), true);
        assert_eq!(or(false, false), false);
    }

    #[test]
    fn xor_test() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(false, false), false);
    }

    #[test]
    fn mux_test() {
        assert_eq!(mux(true, true, true), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, false, false), false);
    }

    #[test]
    fn dmux_test() {
        assert_eq!(dmux(true, true), (false, true));
        assert_eq!(dmux(true, false), (true, false));
        assert_eq!(dmux(false, true), (false, false));
        assert_eq!(dmux(false, false), (false, false));
    }
}
