use charabia::{SeparatorKind, TokenKind};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = TokenKind)]
#[derive(Clone, PartialEq)]
pub enum WasmTokenKind {
    Word,

    /// the token is a stop word, meaning that it can be ignored to optimize
    /// size and performance or be indexed as a Word
    StopWord,

    /// Separate two tokens that are in the same context (same phrase).
    SoftSeparator,

    /// Separate two tokens that are not in the same context (different phrases).
    HardSeparator,
    Unknown,
}

#[wasm_bindgen(js_name = SeparatorKind)]
#[derive(Clone)]
pub enum WasmSeparatorKind {
    /// Separate two tokens that are in the same context (same phrase).
    Soft,

    /// Separate two tokens that are not in the same context (different phrases).
    Hard,
}

impl From<TokenKind> for WasmTokenKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Word => WasmTokenKind::Word,
            TokenKind::StopWord => WasmTokenKind::StopWord,
            TokenKind::Separator(SeparatorKind::Soft) => WasmTokenKind::SoftSeparator,
            TokenKind::Separator(SeparatorKind::Hard) => WasmTokenKind::HardSeparator,
            TokenKind::Unknown => WasmTokenKind::Unknown,
        }
    }
}

impl From<SeparatorKind> for WasmSeparatorKind {
    fn from(kind: SeparatorKind) -> Self {
        match kind {
            SeparatorKind::Soft => WasmSeparatorKind::Soft,
            SeparatorKind::Hard => WasmSeparatorKind::Hard,
        }
    }
}
