![GWN Logo](/media/GWN.png)
# GWN
![Version](https://img.shields.io/badge/dynamic/yaml.svg?color=informational&label=version&query=%24.version&url=https%3A%2F%2Fraw.githubusercontent.com%2Fgwn-lang%2Fgwn%2Fmaster%2Fversion.yaml)
GWN is a functional programming language that will solve all of your problems in life, guaranteed.
Think Python with purity and Haskell without hardness.

## Fizzbuzz in GWN

```php
# FizzBuzz from 1 to 100

fizzBuzz =
    {x | x % 15 == 0 ? "FizzBuzz"
         x % 5 == 0 ? "Buzz"
         x % 3 == 0 ? "Fizz"
         else ? x -> toString}
    
print <- [1..100] -> map <- fizzBuzz
```

See more in [/examples](/examples).

## Features
- Minimal, easy to learn syntax
- Static typing with Hindley-Milner type inference
- Robust pattern matching

## Useful Links
- [Wiki](https://github.com/gwn-lang/gwn/wiki)
- [Questions](https://github.com/gwn-lang/gwn/issues/5)
- [Abuse Report](https://github.com/gwn-lang/gwn/issues/4)
- [Codegolf and Challenges](https://github.com/gwn-lang/gwn/issues/8)
- Docs (Coming Soon...)
