macro_rules! valid {
    ($name:ident, $position: ident, $src: expr, $dst: expr) => {
        #[test]
        fn $name() {
            const FEN_POSITION: &str =
                include_str!(concat!("boards/", stringify!($position), ".fen"));
            let board = Board::from_fen(FEN_POSITION).unwrap();
            board.move_notation($src, $dst).unwrap();
        }
    };
}

macro_rules! invalid {
    ($name:ident, $position:ident, $src: expr, $dst: expr, $exp_err: expr) => {
        #[test]
        fn $name() {
            const FEN_POSITION: &str =
                include_str!(concat!("boards/", stringify!($position), ".fen"));
            let board = Board::from_fen(FEN_POSITION).unwrap();
            let pos = board.move_notation($src, $dst);
            let err = pos.err().unwrap();
            assert_eq!($exp_err, err);
        }
    };
}

// BEGIN TESTS.

mod move_validation {
    use game::{Board, MoveError};

    valid!(cant_check_through_piece, check_through_piece, "f2", "e1");

    invalid!(
        cant_move_pinned_piece,
        pinned_pawn,
        "d2",
        "d3",
        MoveError::PutSelfInCheck
    );

    valid!(can_take_when_pinned, pinned_pawn, "d2", "c3");

    // Test pawn movement.
    valid!(pawn_first_move_one_square, starting, "d2", "d3");
    valid!(pawn_first_move_two_squares, starting, "d2", "d4");
    invalid!(
        pawn_cant_go_backward_white,
        e3,
        "e3",
        "e2",
        MoveError::InvalidMove
    );
    invalid!(
        pawn_cant_go_backward_black,
        e6,
        "e6",
        "e7",
        MoveError::InvalidMove
    );
    invalid!(
        pawn_cant_move_two_after_first_move,
        e3,
        "e3",
        "e5",
        MoveError::InvalidMove
    );
    valid!(pawn_en_passant, en_passant, "e4", "f3");
    invalid!(
        pawn_en_passant_expires,
        en_passant_expired,
        "e4",
        "f3",
        MoveError::InvalidMove
    );
    invalid!(
        pawn_cant_move_side_to_side,
        e3,
        "e3",
        "d3",
        MoveError::InvalidMove
    );

    // Test knight movement
    valid!(knight_movement_1, empty_knight, "e4", "d6");
    valid!(knight_movement_2, empty_knight, "e4", "f6");
    valid!(knight_movement_3, empty_knight, "e4", "c5");
    valid!(knight_movement_4, empty_knight, "e4", "g5");
    valid!(knight_movement_5, empty_knight, "e4", "c3");
    valid!(knight_movement_6, empty_knight, "e4", "g3");
    valid!(knight_movement_7, empty_knight, "e4", "d2");
    valid!(knight_movement_8, empty_knight, "e4", "f2");
    invalid!(
        knight_cant_move_straight,
        empty_knight,
        "e4",
        "e5",
        MoveError::InvalidMove
    );
    invalid!(
        knight_cant_move_diagonally,
        empty_knight,
        "e4",
        "f5",
        MoveError::InvalidMove
    );
    valid!(knight_goes_over_pieces, knight_boxed, "e4", "d6");
}
