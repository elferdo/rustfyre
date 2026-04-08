# About

This is an implementation in rust of the DeJong chaotic map as originally demonstrated in [fyre](https://en.wikipedia.org/wiki/Fyre_(software)).

# Example

Running:

```
cargo run -- -i 3000000 -o dejong.png -a -2.7 -b -0.9 -c -0.86 -d -2.2
```

will produce the following output:
![Sample output of the program rendering the DeJong oscillator](/examples/dejong.png)
