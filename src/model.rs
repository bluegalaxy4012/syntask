use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    id: u32,
    title: String,
    texts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Board {cards: Vec::new()}
    }

    pub fn add_card(&mut self) -> u32 {
        let id = self.cards.len() as u32;
        self.cards.push(Card{id, title: "New Card".to_string(), texts: Vec::new()});
        id
    }

    pub fn remove_card(&mut self, id: u32) -> Option<Card> {
        //find card with that id and remove it

        if let Some(index) = self.cards.iter().position(|c| c.id == id) {
            Some(self.cards.remove(index))
        }
        else
        {
            None
        }

    }

    pub fn edit_title(&mut self, card_id: u32, text: String) {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            card.title = text;
        }
    }

    pub fn add_text(&mut self, card_id: u32, text: String) {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            card.texts.push(text);
        }
    }
    
    pub fn edit_text(&mut self, card_id: u32, text: String, text_index: usize) {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            card.texts[text_index] = text;
        }
    }

    
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    
    AddCard,
    RemoveCard {id: u32},
    EditTitle {card_id: u32, text: String},
    AddText {card_id: u32, text: String},
    EditText {card_id: u32, text: String, text_index: usize},
}

