# Arithmetic Coding in Rust

Hi, this is a implementation of Arithmetic Coding written in Rust

It's the optimization of a homework project. Inspired in [Mark Nelson article](https://marknelson.us/posts/2014/10/19/data-compression-with-arithmetic-coding.html) and nayuki/Reference-arithmetic-coding coded in cpp. Lately reorganised like cgbur/arcode-rs

Actually, there's only a simple flat model that represent symbol probabilities, but many other can be implemented following the same trait

The sample script encodes "El Quijote" in 1.1 MB while it's plain text size is 2.1 MB