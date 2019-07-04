# Andaluh-rs

Transliterate español (spanish) spelling to andaluz proposals

## Table of Contents

- [Description](#description)
- [Usage](#usage)
- [Support](#support)
- [Contributing](#contributing)

## Description

The **Andalusian varieties of [Spanish]** (Spanish: *andaluz*; Andalusian) are spoken in Andalusia, Ceuta, Melilla, and Gibraltar. They include perhaps the most distinct of the southern variants of peninsular Spanish, differing in many respects from northern varieties, and also from Standard Spanish. Further info: https://en.wikipedia.org/wiki/Andalusian_Spanish.

This package introduces transliteration functions to convert *español* (spanish) spelling to andaluz. As there's no official or standard andaluz spelling, andaluh-py is adopting the **EPA proposal (Er Prinzipito Andaluh)**. Further info: https://andaluhepa.wordpress.com. Other andaluz spelling proposals are planned to be added as well.

## Usage

This is a rust library so you can use to transliterate any string:

```rust
use andaluh::epa;

let input = "El veloz murciélago hindú comía feliz cardillo y kiwi. La cigüeña tocaba el saxofón detrás del palenque de paja.";
let output = epa(input).expect("Wrong parser");
assert_eq!(input, output);
```

## Support

Please [open an issue](https://github.com/andalugeeks/andaluh-rs/issues/new) for support.

## Contributing

Please contribute using [Github Flow](https://guides.github.com/introduction/flow/). Create a branch, add commits, and open a pull request.
