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

## Benchmarks

```sh
npm install
npm run benchmark
```

The following benchmarks were run on a MacBook Air (13-inch, M1, 2020) with Node.js v20.13.0.

> Note: These benchmarks were executed in a non-dedicated environment and are provided for reference only. The results may vary depending on hardware performance, system load, and other factors.

```sh
tokenize - 132 bytes - Cj/Cmn x 27,827 ops/sec ±24.70% (75 runs sampled)
segment - 132 bytes - Cj/Cmn x 62,394 ops/sec ±1.59% (95 runs sampled)
tokenize - 132 bytes - Cj/Jpn x 18,466 ops/sec ±3.17% (91 runs sampled)
segment - 132 bytes - Cj/Jpn x 25,273 ops/sec ±0.11% (98 runs sampled)
tokenize - 132 bytes - Latin/Eng x 15,012 ops/sec ±86.56% (56 runs sampled)
segment - 132 bytes - Latin/Eng x 66,696 ops/sec ±3.11% (96 runs sampled)
tokenize - 132 bytes - Latin/Fra x 26,493 ops/sec ±23.67% (73 runs sampled)
segment - 132 bytes - Latin/Fra x 73,449 ops/sec ±1.23% (99 runs sampled)
tokenize - 132 bytes - Hebrew/Heb x 49,838 ops/sec ±1.78% (98 runs sampled)
segment - 132 bytes - Hebrew/Heb x 126,324 ops/sec ±0.16% (100 runs sampled)
tokenize - 132 bytes - Thai/Tha x 58,358 ops/sec ±0.50% (98 runs sampled)
segment - 132 bytes - Thai/Tha x 107,640 ops/sec ±0.12% (100 runs sampled)
tokenize - 132 bytes - Hangul/Kor x 12,729 ops/sec ±5.42% (86 runs sampled)
segment - 132 bytes - Hangul/Kor x 16,715 ops/sec ±0.58% (99 runs sampled)
tokenize - 130 bytes - Greek/Ell x 50,880 ops/sec ±0.51% (95 runs sampled)
segment - 130 bytes - Greek/Ell x 138,118 ops/sec ±2.62% (95 runs sampled)
tokenize - 132 bytes - Khmer/Khm x 19,983 ops/sec ±1.17% (97 runs sampled)
segment - 132 bytes - Khmer/Khm x 36,219 ops/sec ±0.12% (100 runs sampled)
tokenize - 132 bytes - Arabic/Ara x 48,324 ops/sec ±1.64% (98 runs sampled)
segment - 132 bytes - Arabic/Ara x 118,640 ops/sec ±0.21% (99 runs sampled)
tokenize - 134 bytes - Arabic/Vie x 33,425 ops/sec ±1.46% (98 runs sampled)
segment - 134 bytes - Arabic/Vie x 84,432 ops/sec ±0.12% (100 runs sampled)
tokenize - 131 bytes - Latin/Deu x 38,375 ops/sec ±5.86% (84 runs sampled)
segment - 131 bytes - Latin/Deu x 89,398 ops/sec ±0.57% (99 runs sampled)
tokenize - 363 bytes - Cj/Cmn x 13,298 ops/sec ±3.44% (93 runs sampled)
segment - 363 bytes - Cj/Cmn x 21,941 ops/sec ±0.94% (99 runs sampled)
tokenize - 364 bytes - Cj/Jpn x 6,724 ops/sec ±1.93% (92 runs sampled)
segment - 364 bytes - Cj/Jpn x 8,753 ops/sec ±0.12% (99 runs sampled)
tokenize - 363 bytes - Latin/Eng x 13,784 ops/sec ±4.32% (83 runs sampled)
segment - 363 bytes - Latin/Eng x 29,857 ops/sec ±0.16% (98 runs sampled)
tokenize - 363 bytes - Latin/Fra x 12,016 ops/sec ±5.20% (83 runs sampled)
segment - 363 bytes - Latin/Fra x 29,172 ops/sec ±0.64% (100 runs sampled)
tokenize - 365 bytes - Hebrew/Heb x 18,363 ops/sec ±4.90% (90 runs sampled)
segment - 365 bytes - Hebrew/Heb x 49,418 ops/sec ±0.17% (98 runs sampled)
tokenize - 366 bytes - Thai/Tha x 21,969 ops/sec ±3.66% (91 runs sampled)
segment - 366 bytes - Thai/Tha x 45,026 ops/sec ±1.05% (99 runs sampled)
tokenize - 364 bytes - Hangul/Kor x 5,096 ops/sec ±0.81% (91 runs sampled)
segment - 364 bytes - Hangul/Kor x 6,538 ops/sec ±0.71% (96 runs sampled)
tokenize - 364 bytes - Greek/Ell x 12,715 ops/sec ±8.90% (82 runs sampled)
segment - 364 bytes - Greek/Ell x 43,761 ops/sec ±0.12% (100 runs sampled)
tokenize - 327 bytes - Khmer/Khm x 11,885 ops/sec ±3.67% (95 runs sampled)
segment - 327 bytes - Khmer/Khm x 21,975 ops/sec ±0.31% (98 runs sampled)
tokenize - 366 bytes - Arabic/Ara x 12,793 ops/sec ±13.06% (73 runs sampled)
segment - 366 bytes - Arabic/Ara x 36,981 ops/sec ±4.93% (86 runs sampled)
tokenize - 365 bytes - Latin/Vie x 10,048 ops/sec ±6.36% (84 runs sampled)
segment - 365 bytes - Latin/Vie x 30,592 ops/sec ±0.18% (99 runs sampled)
tokenize - 354 bytes - Latin/Deu x 15,218 ops/sec ±5.53% (82 runs sampled)
segment - 354 bytes - Latin/Deu x 37,524 ops/sec ±0.61% (99 runs sampled)
```

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
