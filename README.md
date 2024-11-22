# charabia-js

`charabia-js` is a WebAssembly binding for the [charabia](https://github.com/meilisearch/charabia) multilingual text tokenizer used by [Meilisearch](https://github.com/meilisearch/meilisearch).

## Supported scripts / languages

- Latin
- Latin - German
- Greek
- Cyrillic - Georgian
- Chinese CMN 🇨🇳
- Hebrew 🇮🇱
- Arabic
- Japanese 🇯🇵
- Korean 🇰🇷
- Thai 🇹🇭
- Khmer 🇰🇭

More information about the supported scripts and languages can be found in the [here](https://github.com/meilisearch/charabia#supported-languages).

## Installation

```sh
npm install charabia-js
```

## Usage

### Segmentation

```ts
import { segment } from "charabia-js";

console.log(segment("Hello, world!")); // [ 'Hello', ', ', 'world', '!' ]
console.log(segment("你好，世界！")); // [ '你好', '，', '世界', '！' ]
console.log(segment("Hello, 世界!")); // [ 'Hello', ', ', '世界', '!' ]
```

### Tokenization

```ts
import { tokenize, TokenKind } from "charabia-js";
import assert from "node:assert";

const tokens = tokenize(
  "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F"
);

let token = tokens[0];
assert.equal(token.lemma, "the");
assert.equal(token.kind, TokenKind.Word);

token = tokens[1];
assert.equal(token.lemma, " ");
assert.equal(token.kind, TokenKind.SoftSeparator);

token = tokens[2];
assert.equal(token.lemma, "quick");
assert.equal(token.kind, TokenKind.Word);
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
