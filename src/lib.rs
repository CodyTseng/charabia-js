mod models;

use crate::models::token::WasmToken;
use charabia::{Segment, Tokenize};
use wasm_bindgen::prelude::*;

/// Segments the input text into an array of strings.
///
/// @param {string} text - A string to be segmented.
/// @returns {string[]} An array of strings, where each string is a segment of the input text.
/// @example
/// import { segment } from 'charabia-js';
///
/// segment("Hello, world!") // [ 'Hello', ', ', 'world', '!' ]
/// segment("你好，世界！") // [ '你好', '，', '世界', '！' ]
/// segment("Hello, 世界!") // [ 'Hello', ', ', '世界', '!' ]")
#[wasm_bindgen(skip_jsdoc)]
pub fn segment(text: &str) -> Vec<String> {
    let segments = text.segment_str();
    segments.map(|s| s.to_string()).collect()
}

/// Segments the provided text into tokens, then normalizes and classifies each token.
///
/// @param {string} text - A string to be tokenized.
/// @returns {Token[]} An array of tokens, where each token is a normalized and classified segment of the input text.
/// @example
/// import { tokenize, TokenKind } from 'charabia-js';
/// import assert from 'node:assert';
///
/// const tokens = tokenize(
///   'The quick ("brown") fox can\'t jump 32.3 feet, right? Brr, it\'s 29.3°F'
/// );
///
/// let token = tokens[0];
/// assert.equal(token.lemma, 'the');
/// assert.equal(token.kind, TokenKind.Word);
///
/// token = tokens[1];
/// assert.equal(token.lemma, ' ');
/// assert.equal(token.kind, TokenKind.SoftSeparator);
///
/// token = tokens[2];
/// assert.equal(token.lemma, 'quick');
/// assert.equal(token.kind, TokenKind.Word);
#[wasm_bindgen(skip_jsdoc)]
pub fn tokenize(text: &str) -> Vec<WasmToken> {
    let tokens = text.tokenize();
    tokens.map(WasmToken::from).collect()
}
