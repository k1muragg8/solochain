use crate::data::{Kana, get_hiragana, get_katakana, get_all_kana};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Menu,
    Quiz,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub quiz_mode: Option<QuizMode>,
    pub quiz_state: QuizState,
    pub menu_state: MenuState,
    pub should_exit: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum QuizMode {
    Hiragana,
    Katakana,
    Mixed,
}

pub struct MenuState {
    pub selected_index: usize,
    pub items: Vec<QuizMode>,
}

pub struct QuizState {
    pub current_kana: Option<Kana>,
    pub user_input: String,
    pub score: u32,
    pub total_attempts: u32,
    pub streak: u32,
    pub feedback: Option<Feedback>,
}

#[derive(PartialEq)]
pub enum Feedback {
    Correct,
    Incorrect(String), // The correct answer
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            quiz_mode: None,
            quiz_state: QuizState::new(),
            menu_state: MenuState::new(),
            should_exit: false,
        }
    }

    pub fn start_quiz(&mut self, mode: QuizMode) {
        self.quiz_mode = Some(mode);
        self.quiz_state = QuizState::new();
        self.next_question();
        self.current_screen = CurrentScreen::Quiz;
    }

    pub fn next_question(&mut self) {
        let mode = self.quiz_mode.expect("Quiz mode should be set");
        let pool = match mode {
            QuizMode::Hiragana => get_hiragana(),
            QuizMode::Katakana => get_katakana(),
            QuizMode::Mixed => get_all_kana(),
        };

        let mut rng = thread_rng();
        if let Some(kana) = pool.choose(&mut rng) {
            self.quiz_state.current_kana = Some(kana.clone());
            self.quiz_state.user_input.clear();
            self.quiz_state.feedback = None;
        }
    }

    pub fn submit_answer(&mut self) {
        if let Some(current) = &self.quiz_state.current_kana {
            self.quiz_state.total_attempts += 1;
            if self.quiz_state.user_input.trim().eq_ignore_ascii_case(&current.romaji) {
                self.quiz_state.score += 1;
                self.quiz_state.streak += 1;
                self.quiz_state.feedback = Some(Feedback::Correct);
            } else {
                self.quiz_state.streak = 0;
                self.quiz_state.feedback = Some(Feedback::Incorrect(current.romaji.clone()));
            }
        }
    }
}

impl MenuState {
    fn new() -> Self {
        Self {
            selected_index: 0,
            items: vec![QuizMode::Hiragana, QuizMode::Katakana, QuizMode::Mixed],
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.items.len() - 1;
        }
    }

    pub fn current_selection(&self) -> QuizMode {
        self.items[self.selected_index]
    }
}

impl QuizState {
    fn new() -> Self {
        Self {
            current_kana: None,
            user_input: String::new(),
            score: 0,
            total_attempts: 0,
            streak: 0,
            feedback: None,
        }
    }
}

impl std::fmt::Display for QuizMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuizMode::Hiragana => write!(f, "Hiragana"),
            QuizMode::Katakana => write!(f, "Katakana"),
            QuizMode::Mixed => write!(f, "Mixed"),
        }
    }
}
