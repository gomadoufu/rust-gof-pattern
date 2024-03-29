// 制御するボタンが一つだけついた電子サイコロ
// ボタンを押す、電源ON & サイコロ回転開始
// ボタンを押す、サイコロ停止(出目が決まる)
// ボタンを押す、電源OFF、最初に戻る.
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
//Contexts
enum StateDice {
    PowerOn,
    StopDice,
    PowerOff,
}

trait State {
    fn on_press_button(&self, context: &mut StateContext);
}

//ConcreteStates
struct StatePowerOn;
impl State for StatePowerOn {
    fn on_press_button(&self, context: &mut StateContext) {
        // Something to do for turning on the dice.
        println!("Power on and Shake the dice.");

        context.set_state(StateDice::StopDice);
    }
}

struct StateStop;
impl State for StateStop {
    fn on_press_button(&self, context: &mut StateContext) {
        // Something to do for turning on the dice.
        println!("Stopping the dice.");

        context.set_dice_number(4);

        context.set_state(StateDice::PowerOff);
    }
}

struct StatePowerOff;
impl State for StatePowerOff {
    fn on_press_button(&self, context: &mut StateContext) {
        // Something to do for turning on the dice.
        println!("Power off.");

        context.set_state(StateDice::PowerOn);
    }
}

//Context
#[derive(Debug)]
struct StateContext {
    number: Option<u8>,
    current_state: StateDice,
}

impl StateContext {
    fn new() -> StateContext {
        StateContext {
            number: None,
            current_state: StateDice::PowerOn,
        }
    }

    fn set_state(&mut self, s: StateDice) {
        self.current_state = s;
    }

    fn set_dice_number(&mut self, n: u8) {
        self.number = Some(n)
    }

    fn press_button<'a>(&mut self, hmap: &HashMap<StateDice, Box<dyn State + 'a>>) {
        let b = hmap.get(&self.current_state).unwrap();
        b.on_press_button(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut context = StateContext::new();
        let mut hmap: HashMap<StateDice, Box<dyn State>> = HashMap::new();
        hmap.insert(StateDice::PowerOn, Box::new(StatePowerOn));
        hmap.insert(StateDice::StopDice, Box::new(StateStop));
        hmap.insert(StateDice::PowerOff, Box::new(StatePowerOff));

        context.press_button(&hmap);
        assert_eq!(context.current_state, StateDice::StopDice);
        assert_eq!(context.number, None);

        context.press_button(&hmap);
        assert_eq!(context.current_state, StateDice::PowerOff);
        assert_eq!(context.number, Some(4));

        context.press_button(&hmap);
        assert_eq!(context.current_state, StateDice::PowerOn);
        assert_eq!(context.number, Some(4));
    }
}
