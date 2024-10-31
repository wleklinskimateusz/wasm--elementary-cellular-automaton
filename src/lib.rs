use elementary_cellular_automaton::automaton::Automaton;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CellularAutomaton {
    automaton: Automaton,
    initial_state: u128,
}

#[wasm_bindgen]
impl CellularAutomaton {
    pub fn new(rule: u8, initial_state: Vec<u8>) -> CellularAutomaton {
        let to_number: u128 = initial_state
            .iter()
            .fold(0, |acc, &bit| (acc << 1) | bit as u128);
        CellularAutomaton {
            automaton: Automaton::new(rule, to_number),
            initial_state: to_number,
        }
    }

    pub fn step(&mut self) {
        self.automaton.update();
    }

    pub fn get_state(&self) -> Vec<u8> {
        self.automaton.to_vector()
    }

    pub fn set_rule(&mut self, rule: u8) {
        self.automaton.rule = rule;
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
        let automaton = CellularAutomaton::new(30, vec![1; 128]);
        assert_eq!(automaton.automaton.rule, 30);
    }

    #[test]
    fn test_step_rule_0() {
        let mut automaton = CellularAutomaton::new(0, vec![1; 128]);
        automaton.step();
        assert_eq!(automaton.get_state(), vec![0; 128]);
    }

    #[test]
    fn test_step_rule_30() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128]);
        automaton.step();
        // For rule 30, when all cells are 1, the next state will be all 0s except the left edge
        let mut expected = vec![0; 128];
        expected[0] = 1;
        assert_eq!(automaton.get_state(), expected);
    }

    #[test]
    fn test_reset() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128]);
        automaton.step();
        automaton.reset();
        assert_eq!(automaton.get_state(), vec![1; 128]);
    }

    #[test]
    fn test_set_rule() {
        let mut automaton = CellularAutomaton::new(30, vec![1; 128]);
        automaton.set_rule(0);
        assert_eq!(automaton.automaton.rule, 0);
    }
}
