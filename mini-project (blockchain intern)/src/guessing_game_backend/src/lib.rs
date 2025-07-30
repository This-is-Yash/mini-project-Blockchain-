use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct GuessResult {
    correct: bool,
    message: String,
    guess: u8,
}

thread_local! {
    static SECRET_NUMBER: RefCell<u8> = RefCell::new(7); // can randomize on deploy
    static GUESSES: RefCell<HashMap<String, u8>> = RefCell::new(HashMap::new());
}

#[update]
fn submit_guess(guess: u8) -> GuessResult {
    let caller = ic_cdk::caller().to_text();
    GUESSES.with(|g| g.borrow_mut().insert(caller.clone(), guess));

    SECRET_NUMBER.with(|secret| {
        let number = *secret.borrow();
        if guess == number {
            GuessResult {
                correct: true,
                message: format!("🎉 Correct! {} was the secret number!", guess),
                guess,
            }
        } else {
            GuessResult {
                correct: false,
                message: format!("❌ Wrong guess. Try again!"),
                guess,
            }
        }
    })
}

#[query]
fn get_guesses() -> HashMap<String, u8> {
    GUESSES.with(|g| g.borrow().clone())
}
