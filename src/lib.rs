use elementary_cellular_automaton::automaton::Automaton;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CellularAutomaton {
    automaton: Automaton,
    initial_state: u128,
}

fn vec_to_number(vec: Vec<u8>) -> u128 {
    vec.iter().fold(0, |acc, &bit| (acc << 1) | bit as u128)
}

#[wasm_bindgen]
impl CellularAutomaton {
    pub fn new(rule: u8, initial_state: Vec<u8>, periodic_boundary: bool) -> CellularAutomaton {
        let initial_state_number = vec_to_number(initial_state);
        CellularAutomaton {
            automaton: Automaton::new(rule, initial_state_number, periodic_boundary),
            initial_state: initial_state_number,
        }
    }

    pub fn step(&mut self) {
        self.automaton.update();
    }

    pub fn get_state(&self) -> Vec<u8> {
        self.automaton.to_list().to_vec()
    }

    pub fn set_rule(&mut self, rule: u8) {
        self.automaton.rule = rule;
    }

    pub fn set_initial_state(&mut self, initial_state: Vec<u8>) {
        self.initial_state = vec_to_number(initial_state);
        self.automaton.fields = self.initial_state;
    }

    pub fn set_periodic_boundary(&mut self, periodic_boundary: bool) {
        self.automaton.periodic_boundary = periodic_boundary;
    }

    pub fn reset(&mut self) {
        self.automaton.fields = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_new() {
        let automaton = CellularAutomaton::new(30, vec![1; 128], false);
        assert_eq!(automaton.automaton.rule, 30);
    }

    #[test]
    fn test_step_rule_0() {
        let mut automaton = CellularAutomaton::new(0, vec![1; 128], false);
        automaton.step();
        assert_eq!(automaton.get_state(), vec![0; 128]);
    }

    #[test]
    fn test_step_rule_30() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128], false);
        automaton.step();
        // For rule 30, when all cells are 1, the next state will be all 0s except the left edge
        let mut expected = vec![0; 128];
        expected[0] = 1;
        assert_eq!(automaton.get_state(), expected);
    }

    #[test]
    fn test_reset() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128], false);
        automaton.step();
        automaton.reset();
        assert_eq!(automaton.get_state(), vec![1; 128]);
    }

    #[test]
    fn test_set_rule() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128], false);
        automaton.set_rule(0);
        assert_eq!(automaton.automaton.rule, 0);
    }

    #[test]
    fn test_set_initial_state() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128], false);
        automaton.set_initial_state(vec![0; 128]);
        assert_eq!(automaton.initial_state, 0);
        assert_eq!(automaton.get_state(), vec![0; 128]);
    }

    #[test]
    fn test_periodic_boundary() {
        let mut automaton = CellularAutomaton::new(128, vec![1; 128], false);
        automaton.step();
        let mut expected = vec![1; 128];
        expected[0] = 0;
        expected[127] = 0;

        assert_eq!(automaton.get_state(), expected);

        let mut automaton = CellularAutomaton::new(128, vec![1; 128], true);
        automaton.step();
        assert_eq!(automaton.get_state(), vec![1; 128]);
    }

    #[test]
    fn test_set_periodic_boundary() {
        let mut automaton = CellularAutomaton::new(128, vec![1; 128], false);
        automaton.set_periodic_boundary(true);
        assert_eq!(automaton.automaton.periodic_boundary, true);
    }
}
