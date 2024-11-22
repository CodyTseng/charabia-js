use super::{
    kind::{WasmSeparatorKind, WasmTokenKind},
    language::WasmLanguage,
    script::WasmScript,
};
use charabia::Token;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Token)]
pub struct WasmToken {
    token: Box<Token<'static>>,
}

#[wasm_bindgen(js_class = Token)]
impl WasmToken {
    /// The normalized lemma.
    #[wasm_bindgen(getter)]
    pub fn lemma(&self) -> String {
        self.token.lemma.to_string()
    }

    /// Kind of the Token assigned by the classifier.
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> WasmTokenKind {
        self.token.kind.into()
    }

    /// Index of the first character of the original lemma.
    #[wasm_bindgen(getter, js_name = charStart)]
    pub fn char_start(&self) -> usize {
        self.token.char_start
    }

    /// Index of the last character of the original lemma.
    #[wasm_bindgen(getter, js_name = charEnd)]
    pub fn char_end(&self) -> usize {
        self.token.char_end
    }

    /// Index of the first byte of the original lemma.
    #[wasm_bindgen(getter, js_name = byteStart)]
    pub fn byte_start(&self) -> usize {
        self.token.byte_start
    }

    /// Index of the last byte of the original lemma.
    #[wasm_bindgen(getter, js_name = byteEnd)]
    pub fn byte_end(&self) -> usize {
        self.token.byte_end
    }

    /// Script of the Token.
    #[wasm_bindgen(getter)]
    pub fn script(&self) -> WasmScript {
        self.token.script.into()
    }

    /// Language of the Token.
    #[wasm_bindgen(getter)]
    pub fn language(&self) -> Option<WasmLanguage> {
        self.token.language.map(|lang| lang.into())
    }

    /// The length in bytes of the normalized lemma.
    #[wasm_bindgen(getter, js_name = byteLen)]
    pub fn byte_len(&self) -> usize {
        self.token.byte_len()
    }

    /// The length in bytes of the original lemma.
    #[wasm_bindgen(getter, js_name = originalByteLen)]
    pub fn original_byte_len(&self) -> usize {
        self.token.original_byte_len()
    }

    /// The count of characters of the normalized lemma.
    #[wasm_bindgen(getter, js_name = charCount)]
    pub fn char_count(&self) -> usize {
        self.token.char_count()
    }

    /// The count of characters of the original lemma.
    #[wasm_bindgen(getter, js_name = originalCharCount)]
    pub fn original_char_count(&self) -> usize {
        self.token.original_char_count()
    }

    /// Whether the token is a word. A token is considered as a word if it's not
    /// a separator nor a stop word.
    #[wasm_bindgen(getter, js_name = isWord)]
    pub fn is_word(&self) -> bool {
        self.token.is_word()
    }

    /// Whether the token is a stop word.
    #[wasm_bindgen(getter, js_name = isStopWord)]
    pub fn is_stopword(&self) -> bool {
        self.token.is_stopword()
    }

    /// Whether the token is a separator.
    #[wasm_bindgen(getter, js_name = isSeparator)]
    pub fn is_separator(&self) -> bool {
        self.token.is_separator()
    }

    /// The kind of the separator if the token is a separator.
    #[wasm_bindgen(getter, js_name = separatorKind)]
    pub fn separator_kind(&self) -> Option<WasmSeparatorKind> {
        self.token.separator_kind().map(|kind| kind.into())
    }
}

impl<'o> From<Token<'o>> for WasmToken {
    fn from(token: Token<'o>) -> Self {
        let static_token = Token {
            lemma: Cow::Owned(token.lemma.into_owned()),
            kind: token.kind,
            char_start: token.char_start,
            char_end: token.char_end,
            byte_start: token.byte_start,
            byte_end: token.byte_end,
            char_map: token.char_map,
            script: token.script,
            language: token.language,
        };

        WasmToken {
            token: Box::new(static_token),
        }
    }
}
