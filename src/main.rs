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
        let mut out_sum = 0;

        for hand_card in &self.cards {
            out_sum += match hand_card {
                Card:: Two => 2,
                Card:: Three => 3,
                Card:: Four => 4,
                Card:: Five => 5,
                Card:: Six => 6,
                Card:: Seven => 7,
                Card:: Eight => 8,
                Card:: Nine => 9,
                Card:: Jack | Card::Queen | Card::King => 10,
                Card:: Ace => {
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
