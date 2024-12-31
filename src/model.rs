// model.rs
use crate::utilities::current_unix_time;

pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
    pub subdecks: Option<Vec<Deck>>,
}

pub struct Card {
    pub front: String,
    pub back: String,
    pub notes: Option<String>,
    pub examples: Option<Vec<ExampleSentence>>,
    pub explanation: Option<String>,
    pub history: Option<Vec<ReviewHistory>>,
    pub next_review: Option<u64>,
    pub ease_factor: Option<u64>,
}

pub struct ExampleSentence {
    pub sentence: String,
    pub translation: String,
}

pub struct ReviewHistory {
     date: u64, // UNIX timestamp
     remembered: bool,
 }

 impl Card {
    pub fn calculate_next_review(&mut self, current_time: u64, remembered: bool) -> u64 {
        let random_factor = (0.8 + (rand::random::<f64>() * 0.4)) as u64;
        
        if remembered {
            self.ease_factor = Some((self.ease_factor.unwrap() as f64 * 1.5) as u64);
            self.next_review = Some(current_time + self.ease_factor.unwrap_or(0) + random_factor);
        } else {
            self.ease_factor = Some(60);
            self.next_review = Some(current_time + 60 + random_factor);
        }
        
        self.next_review.expect("Could not set next review.")
    }

    pub fn initialize_review_data(&mut self) {
        if self.next_review.is_none() {
            self.next_review = Some(current_unix_time() as u64);
        }
        if self.ease_factor.is_none() {
            self.ease_factor = Some(60);
        }
        if self.history.is_none() {
            self.history = Some(Vec::new());
        }
    }
 }


 pub fn create_example_deck() -> Deck { 
    Deck {
        name: String::from("Korean Basics"),
        cards: vec![
        Card {
            front: String::from("안녕하세요"),
            back: String::from("Hello (formal)"),
            notes: Some(String::from("Formal greeting used in most situations")),
            examples: Some(vec![
                ExampleSentence {
                    sentence: String::from("안녕하세요, 처음 뵙겠습니다"),
                    translation: String::from("Hello, nice to meet you"),
                }
                ]),
            explanation: Some(String::from("Composed of 안녕 (peace) + 하다 (to do) + honorific suffix 세요")),
            history: Some(vec![]),
            next_review: Some(0),
            ease_factor: Some(60),
        },
        Card {
            front: String::from("감사합니다"),
            back: String::from("Thank you (formal)"),
            notes: Some(String::from("Standard polite way to express gratitude")),
            examples: Some(vec![
                ExampleSentence {
                    sentence: String::from("도와주셔서 감사합니다"),
                    translation: String::from("Thank you for helping me"),
                }
                ]),
            explanation: Some(String::from("감사 (gratitude) + 하다 (to do) + honorific suffix 니다")),
            history: Some(vec![]),
            next_review: Some(0),
            ease_factor: Some(60),
        },
        Card {
            front: String::from("바나나"),
            back: String::from("Banana"),
            notes: Some(String::from("Loan word from English")),
            examples: Some(vec![
                ExampleSentence {
                    sentence: String::from("바나나를 먹고 싶어요"),
                    translation: String::from("I want to eat a banana"),
                }
                ]),
            explanation: Some(String::from("Korean phonetic adaptation of English 'banana'")),
            history: Some(vec![]),
            next_review: Some(0),
            ease_factor: Some(60),
        },
        Card {
            front: String::from("학생"),
            back: String::from("Student"),
            notes: Some(String::from("Common noun for student/learner")),
            examples: Some(vec![
                ExampleSentence {
                    sentence: String::from("저는 한국어 학생입니다"),
                    translation: String::from("I am a Korean language student"),
                }
                ]),
            explanation: Some(String::from("Sino-Korean word: 학 (learn) + 생 (life)")),
            history: Some(vec![]),
            next_review: Some(0),
            ease_factor: Some(60),
        },
        Card {
            front: String::from("고맙습니다"),
            back: String::from("Thank you (semi-formal)"),
            notes: Some(String::from("Slightly less formal than 감사합니다")),
            examples: Some(vec![
                ExampleSentence {
                    sentence: String::from("선물 고맙습니다"),
                    translation: String::from("Thank you for the gift"),
                }
                ]),
            explanation: Some(String::from("고맙다 (to be grateful) + polite ending 습니다")),
            history: Some(vec![]),
            next_review: Some(0),
            ease_factor: Some(60),
        },
        ],
        subdecks: Some(vec![
            Deck {
                name: String::from("Hanja Basics"),
                cards: vec![
                Card {
                    front: String::from("人 (인)"),
                    back: String::from("Person"),
                    notes: Some(String::from("One of the most common hanja characters")),
                    examples: Some(vec![
                        ExampleSentence {
                            sentence: String::from("外國人 (외국인)"),
                            translation: String::from("Foreigner (outside-country-person)"),
                        }
                        ]),
                    explanation: Some(String::from("Pictograph of a person walking. Used in words like 人間 (인간, human), 日本人 (일본인, Japanese person)")),
                    history: Some(vec![]),
                    next_review: Some(0),
                    ease_factor: Some(60),
                },
                Card {
                    front: String::from("山 (산)"),
                    back: String::from("Mountain"),
                    notes: Some(String::from("Basic nature-related hanja")),
                    examples: Some(vec![
                        ExampleSentence {
                            sentence: String::from("富士山 (후지산)"),
                            translation: String::from("Mount Fuji"),
                        }
                        ]),
                    explanation: Some(String::from("Pictograph of three mountain peaks. Used in words like 山水 (산수, landscape)")),
                    history: Some(vec![]),
                    next_review: Some(0),
                    ease_factor: Some(60),
                },
                Card {
                    front: String::from("日 (일)"),
                    back: String::from("Sun/Day"),
                    notes: Some(String::from("Fundamental hanja for time-related concepts")),
                    examples: Some(vec![
                        ExampleSentence {
                            sentence: String::from("日曜日 (일요일)"),
                            translation: String::from("Sunday"),
                        }
                        ]),
                    explanation: Some(String::from("Pictograph of the sun. Used in words like 日本 (일본, Japan), 今日 (금일, today)")),
                    history: Some(vec![]),
                    next_review: Some(0),
                    ease_factor: Some(60),
                }
                ],
                subdecks: None,
            }
            ]),
    }
 }