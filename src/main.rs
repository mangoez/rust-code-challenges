#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        // TODO: implement this method
        use Card::*;

        let mut out_sum = 0;

        for hand_card in &self.cards {
            match hand_card {
                Two => out_sum += 2,
                Three => out_sum += 3,
                Four => out_sum += 4,
                Five => out_sum += 5,
                Six => out_sum += 6,
                Seven => out_sum += 7,
                Eight => out_sum += 8,
                Nine => out_sum += 9,
                Jack | Card::Queen | Card::King => out_sum += 10,
                Ace => {
                    if out_sum + 10 >= 21 {
                        out_sum += 1;
                    } else {
                        out_sum += 11;
                    }
                }
            }
        } 
        out_sum
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
}


#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);
    
    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
