use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct QuestionID(u8);

#[derive(Debug)]
struct Question {
    id: QuestionID,
    content: String,
}

struct Answer {
    question_id: QuestionID,
    content: String,
}

struct Form {
    questions: Vec<Question>,
    user_answer: HashMap<QuestionID, Answer>,
}

impl Form {
    fn new() -> Self {
        Form {
            questions: vec![
                Question {
                    id: QuestionID(1),
                    content: "名前".to_string(),
                },
                Question {
                    id: QuestionID(2),
                    content: "住所".to_string(),
                },
            ],
            user_answer: HashMap::new(),
        }
    }
    fn get_first(&self) -> &Question {
        let q = &self.questions[0];
        q
    }
    fn answer(&mut self, user_answer: String, current_question_id: &QuestionID) {
        let answer = Answer {
            question_id: *current_question_id,
            content: user_answer
        };
        &self.user_answer.insert(*current_question_id, answer);
    }
}
fn main() {
    let mut user_form = Form::new();
    let q = user_form.get_first();
    // TODO: fix
    user_form.answer("user_answer".to_string(), &q.id);
    // println!("first q is {:?}", &q);
}
