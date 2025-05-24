//! Card and deck types for deckbuilder games.
//!
//! Provides card data structures, deck management, and game context for turn-based card games.
//!
//! # Example
//!
//! ```rust
//! use deckbuilder_eng::card::*;
//!
//! // Create a card
//! let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
//!
//! // Create a deck with cards
//! let mut deck = Deck::new(vec![card.clone()]);
//!
//! // Draw a card
//! let drawn = deck.draw();
//!
//! // Game context
//! let mut ctx = GameContext::new(30, 30);
//! ctx.deal_damage(5);
//! ctx.heal(3);
//! ```
//!
//! # Details
//!
//! - `Card` is the basic card data structure.
//! - `Deck` manages draw/discard piles and card operations.
//! - `GameContext` tracks player/enemy health, energy, and turn.
//! - `Playable` trait allows custom card effects.
//! - See each struct and function's documentation for more.

/// Unique identifier for each card.
pub type CardId = u32;

/// Card type/category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}

/// Basic card data.
#[derive(Debug, Clone)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub card_type: CardType,
}

impl Card {
    /// Creates a new `Card`.
    ///
    /// # Parameters
    /// - `id`: unique identifier for the card
    /// - `name`: display name of the card
    /// - `description`: text describing card effects
    /// - `cost`: energy cost to play the card
    /// - `card_type`: category of the card (Attack, Skill, Power)
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType};
    /// let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
    /// ```
    pub fn new(
        id: CardId,
        name: impl Into<String>,
        description: impl Into<String>,
        cost: u32,
        card_type: CardType,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            cost,
            card_type,
        }
    }
}

/// Deck holding draw and discard piles.
pub struct Deck {
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
}

impl Deck {
    /// Creates a new deck containing the given cards in the draw pile.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
    /// let deck = Deck::new(vec![card]);
    /// ```
    pub fn new(cards: Vec<Card>) -> Self {
        Self {
            draw_pile: cards,
            discard_pile: Vec::new(),
        }
    }

    /// Shuffles the discard pile back into the draw pile and reverses the order.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// deck.shuffle();
    /// ```
    pub fn shuffle(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        self.draw_pile.reverse();
    }

    /// Draws a card from the draw pile, shuffling if the draw pile is empty.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let card = deck.draw();
    /// ```
    pub fn draw(&mut self) -> Option<Card> {
        if let Some(card) = self.draw_pile.pop() {
            Some(card)
        } else {
            self.shuffle();
            self.draw_pile.pop()
        }
    }

    /// Discards a card by moving it into the discard pile.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
    /// deck.discard(card);
    /// ```
    pub fn discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    /// Draws up to `count` cards, stopping early if the deck is exhausted.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let cards = deck.draw_multiple(2);
    /// ```
    pub fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(c) = self.draw() {
                drawn.push(c);
            } else {
                break;
            }
        }
        drawn
    }

    /// Peeks at the top `count` cards without removing them.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let deck = Deck::new(vec![]);
    /// let top_cards = deck.peek(2);
    /// ```
    pub fn peek(&self, count: usize) -> Vec<&Card> {
        self.draw_pile.iter().rev().take(count).collect()
    }

    /// Mills the top `count` cards: moves them from the draw pile to the discard pile and returns them.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let milled = deck.mill(2);
    /// ```
    pub fn mill(&mut self, count: usize) -> Vec<Card> {
        let mut milled = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw_pile.pop() {
                milled.push(card.clone());
                self.discard_pile.push(card);
            } else {
                break;
            }
        }
        milled
    }

    /// Searches the draw pile for the first card matching `predicate`, removes and returns it.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let found = deck.search(|c| c.name == "Strike");
    /// ```
    pub fn search<F>(&mut self, predicate: F) -> Option<Card>
    where
        F: Fn(&Card) -> bool,
    {
        if let Some(pos) = self.draw_pile.iter().position(|c| predicate(c)) {
            Some(self.draw_pile.remove(pos))
        } else {
            None
        }
    }

    /// Moves the given `card` to the bottom of the draw pile.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Card, CardType, Deck};
    /// let mut deck = Deck::new(vec![]);
    /// let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
    /// deck.move_to_bottom(card);
    /// ```
    pub fn move_to_bottom(&mut self, card: Card) {
        self.draw_pile.insert(0, card);
    }
}

/// Game context holding player/enemy health, energy, and turn.
pub struct GameContext {
    pub player_health: i32,
    pub enemy_health: i32,
    pub energy: u32,
    pub turn: u32, // current turn number
}

impl GameContext {
    /// Creates a new game context with specified player and enemy health.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// let ctx = GameContext::new(30, 30);
    /// ```
    pub fn new(player_health: i32, enemy_health: i32) -> Self {
        Self {
            player_health,
            enemy_health,
            energy: 0,
            turn: 1,
        }
    }

    /// Deals damage to the enemy, reducing their health by `amount`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// # let mut ctx = GameContext::new(30, 30);
    /// ctx.deal_damage(5);
    /// ```
    pub fn deal_damage(&mut self, amount: i32) {
        self.enemy_health -= amount;
    }
    /// Heals the player, increasing their health by `amount`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// # let mut ctx = GameContext::new(30, 30);
    /// ctx.heal(3);
    /// ```
    pub fn heal(&mut self, amount: i32) {
        self.player_health += amount;
    }

    /// Attempts to spend `amount` energy; returns `true` if successful.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// # let mut ctx = GameContext::new(30, 30);
    /// let ok = ctx.spend_energy(2);
    /// ```
    pub fn spend_energy(&mut self, amount: u32) -> bool {
        if self.energy >= amount {
            self.energy -= amount;
            true
        } else {
            false
        }
    }

    /// Returns `true` if either the player or the enemy has zero or negative health.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// # let ctx = GameContext::new(0, 10);
    /// let over = ctx.is_game_over();
    /// ```
    pub fn is_game_over(&self) -> bool {
        self.player_health <= 0 || self.enemy_health <= 0
    }

    /// Starts a new turn, incrementing the turn counter and resetting energy to `max_energy`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::GameContext;
    /// # let mut ctx = GameContext::new(30, 30);
    /// ctx.new_turn(3);
    /// ```
    pub fn new_turn(&mut self, max_energy: u32) {
        self.turn += 1;
        self.energy = max_energy;
    }
}

/// Trait for card effects that can be played/applied.
pub trait Playable {
    /// Play the card effect, modifying the game context.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::card::{Playable, GameContext};
    /// # struct MyCard;
    /// # impl Playable for MyCard {
    /// #   fn play(&self, ctx: &mut GameContext) { ctx.deal_damage(1); }
    /// # }
    /// # let mut ctx = GameContext::new(10, 10);
    /// # let card = MyCard;
    /// card.play(&mut ctx);
    /// ```
    fn play(&self, ctx: &mut GameContext);
}

/// Example attack card that deals damage.
pub struct AttackCard {
    pub card: Card,
    pub damage: i32,
}

impl Playable for AttackCard {
    fn play(&self, ctx: &mut GameContext) {
        ctx.deal_damage(self.damage);
    }
}

/// Example heal card that heals the player or an ally.
pub struct HealCard {
    pub card: Card,
    pub heal_amount: i32,
}

impl Playable for HealCard {
    fn play(&self, ctx: &mut GameContext) {
        ctx.heal(self.heal_amount);
    }
}

/// Card that triggers multiple `Playable` effects in sequence.
pub struct CompoundCard {
    pub card: Card,
    pub effects: Vec<Box<dyn Playable>>,
}

impl Playable for CompoundCard {
    fn play(&self, ctx: &mut GameContext) {
        for effect in &self.effects {
            effect.play(ctx);
        }
    }
}
