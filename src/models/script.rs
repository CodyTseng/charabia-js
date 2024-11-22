use charabia::Script;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Script)]
#[derive(Clone)]
pub enum WasmScript {
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Kannada,
    Khmer,
    Latin,
    Malayalam,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai,
    Cj,
    Other,
}

impl From<Script> for WasmScript {
    fn from(script: Script) -> Self {
        match script {
            Script::Arabic => WasmScript::Arabic,
            Script::Armenian => WasmScript::Armenian,
            Script::Bengali => WasmScript::Bengali,
            Script::Cyrillic => WasmScript::Cyrillic,
            Script::Devanagari => WasmScript::Devanagari,
            Script::Ethiopic => WasmScript::Ethiopic,
            Script::Georgian => WasmScript::Georgian,
            Script::Greek => WasmScript::Greek,
            Script::Gujarati => WasmScript::Gujarati,
            Script::Gurmukhi => WasmScript::Gurmukhi,
            Script::Hangul => WasmScript::Hangul,
            Script::Hebrew => WasmScript::Hebrew,
            Script::Kannada => WasmScript::Kannada,
            Script::Khmer => WasmScript::Khmer,
            Script::Latin => WasmScript::Latin,
            Script::Malayalam => WasmScript::Malayalam,
            Script::Myanmar => WasmScript::Myanmar,
            Script::Oriya => WasmScript::Oriya,
            Script::Sinhala => WasmScript::Sinhala,
            Script::Tamil => WasmScript::Tamil,
            Script::Telugu => WasmScript::Telugu,
            Script::Thai => WasmScript::Thai,
            Script::Cj => WasmScript::Cj,
            Script::Other => WasmScript::Other,
        }
    }
}
