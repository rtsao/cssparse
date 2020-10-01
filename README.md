# cssparse

A CSS parser powered by the style crate from Servo.

## Why?

I was unable to find a single Node.js library that could reliably parse shorthand CSS property values without using Web APIs.

My desired use case was expanding shorthand properties into their constituent longform properties. For example:

<!-- prettier-ignore -->
```css
background: center / contain no-repeat url("../foo.svg"), #eee 35% url("../bar.png");
```

The libraries I tested all failed to address my use case, for the following reasons:

1. Doesn't parse property values at all
   - [`postcss@8.1.1`](https://github.com/postcss/postcss)
2. Doesn't support `background` property values at all
   - [`inline-style-expand-shorthand@1.2.0`](https://github.com/robinweser/inline-style-expand-shorthand)
   - [`css-shorthand-expanders@1.1.0`](https://github.com/kripod/css-shorthand-expanders)
3. Merely tokenizes `background` property values, failing to parse the value into sufficiently useful information
   - [`tree-sitter-css@0.16.0`](https://github.com/tree-sitter/tree-sitter-css)
   - [`postcss-value-parser@4.1.0`](https://github.com/TrySound/postcss-value-parser)
   - [`postcss-values-parser@4.0.0`](https://github.com/shellscape/postcss-values-parser)
4. Fails when parsing valid `background` values (such as the example above)
   - [`css-shorthand-expand@1.2.0`](https://github.com/kapetan/css-shorthand-expand)
   - [`css-property-parser@1.0.6`](https://github.com/mahirshah/css-property-parser)
5. Relies on browser-specific APIs such as [`CSSStyleDeclaration`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)
   - [`css-background-parser@0.1.0`](https://github.com/gilmoreorless/css-background-parser)

After many failed attempts, I have come to the conclusion that the most accurate and reliable way to parse CSS is to simply use the parser from a production-grade web browser. The style crate from Servo is a good choice because it has been [integrated into Firefox](https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo/).
