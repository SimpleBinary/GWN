![GWN Logo](/media/GWN.png)
# GWN
GWN is a functional programming language that will solve all of your problems in life, guaranteed.
Think Python with purity and Haskell without hardness.

```php
# FizzBuzz from 1 to 100

fizzBuzz =
    {x | x % 15 == 0 ? "FizzBuzz"
       | x % 5 == 0 ?  "Buzz"
       | x % 3 == 0 ?  "Fizz"
       | else ?        x -> toString}
    
print <- [1..100] -> map <- fizzBuzz
```

## Features
- Minimal, easy to learn syntax
- Static typing with Hindley-Milner type inference
- Robust pattern matching
