#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Circle,
    Cross,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Circle => write!(f, "Circle"),
            State::Cross => write!(f, "Cross"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    State(Option<State>),
    Board(Box<Board>),
}

impl Element {
    pub fn get_state(&self) -> Option<State> {
        match self {
            &Element::State(state) => state,
            Element::Board(board) => board.get_winner(),
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::State(None)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    pub elements: [[Element; 3]; 3],
}

impl Board {
    pub fn is_stalemate(&self) -> bool {
        self.elements.iter().flatten().all(|state| match state {
            Element::State(state) => state.is_some(),
            Element::Board(board) => board.is_stalemate() || board.get_winner().is_some(),
        }) && self.get_winner().is_none()
    }

    pub fn get_winner(&self) -> Option<State> {
        let states: [[Option<State>; 3]; 3] =
            std::array::from_fn(|x| std::array::from_fn(|y| self.elements[x][y].get_state()));

        fn check_winner(state: State, states: &[[Option<State>; 3]; 3]) -> bool {
            // Check vertical
            for column in states {
                if column.iter().all(|&s| s == Some(state)) {
                    return true;
                }
            }

            // Check horizontal
            for y in 0..3 {
                let mut won = true;
                #[allow(clippy::needless_range_loop)]
                for x in 0..3 {
                    if states[x][y] != Some(state) {
                        won = false;
                        break;
                    }
                }
                if won {
                    return true;
                }
            }

            // Check diagonal right
            {
                let mut won = true;
                #[allow(clippy::needless_range_loop)]
                for i in 0..3 {
                    if states[i][i] != Some(state) {
                        won = false;
                        break;
                    }
                }
                if won {
                    return true;
                }
            }

            // Check diagonal left
            {
                let mut won = true;
                for i in 0..3 {
                    if states[2 - i][i] != Some(state) {
                        won = false;
                        break;
                    }
                }
                if won {
                    return true;
                }
            }

            false
        }

        if check_winner(State::Circle, &states) {
            Some(State::Circle)
        } else if check_winner(State::Cross, &states) {
            Some(State::Cross)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing() {
        let board = Board {
            elements: [
                [
                    Element::State(None),
                    Element::State(None),
                    Element::State(None),
                ],
                [
                    Element::State(None),
                    Element::State(None),
                    Element::State(None),
                ],
                [
                    Element::State(None),
                    Element::State(None),
                    Element::State(None),
                ],
            ],
        };
        assert!(board.get_winner().is_none());
        assert!(!board.is_stalemate());
    }

    #[test]
    fn cross_horizontal_win() {
        let board = Board {
            elements: [
                [
                    Element::State(Some(State::Cross)),
                    Element::State(None),
                    Element::State(None),
                ],
                [
                    Element::State(Some(State::Cross)),
                    Element::State(None),
                    Element::State(None),
                ],
                [
                    Element::State(Some(State::Cross)),
                    Element::State(None),
                    Element::State(None),
                ],
            ],
        };
        assert_eq!(board.get_winner(), Some(State::Cross));
        assert!(!board.is_stalemate());
    }

    #[test]
    fn circle_diagonal_win() {
        let board = Board {
            elements: [
                [
                    Element::State(Some(State::Cross)),
                    Element::State(None),
                    Element::State(None),
                ],
                [
                    Element::State(None),
                    Element::State(Some(State::Cross)),
                    Element::State(None),
                ],
                [
                    Element::State(None),
                    Element::State(None),
                    Element::State(Some(State::Cross)),
                ],
            ],
        };
        assert_eq!(board.get_winner(), Some(State::Cross));
        assert!(!board.is_stalemate());
    }
}
