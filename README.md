= Purpose
To explore interop between python and Rust.

= Why?
- Rust code runs much faster and uses less resources.
- But Rust is harder to develop. Iteration on a Rust project is commonly slower than in python.
- Ideal project development may be to write the body in python but to optimize parts in Rust.

= Resources
- [pyo3](https://github.com/PyO3/pyo3)
- [Supercharge Python with Rust](https://www.youtube.com/watch?v=zepPZ6MFiGs)
- [Carl Kadie - Nine Rules for Writing Python Extensions in Rust](https://www.youtube.com/watch?v=B6E0Jb6yj34)
- [Create a Python Package with Super-Fast Rust Code in 3 Steps](https://towardsdatascience.com/create-a-python-package-with-super-fast-rust-code-in-3-steps-a27389629beb)

# Steps
1. Basic implementation in python
```
❯ time python3 fibonacci.py 20
20: 6765
python3 fibonacci.py 20  0.02s user 0.02s system 7% cpu 0.534 total25: 75025
❯ time python3 fibonacci.py 25
25: 75025
python3 fibonacci.py 25  0.05s user 0.02s system 11% cpu 0.574 total
❯ time python3 fibonacci.py 30
30: 832040
python3 fibonacci.py 30  0.33s user 0.02s system 40% cpu 0.860 total
```

2. Set up maturin
```
> pyenv virtualenv 3.11.4 interop-fib
> pyenv activate interop-fib
> echo maturin > requirements.txt
> pip install -r requirements.txt
```

3. Add pyo3 to crate

4. Initial build
```
> cargo build
```

5. Initial Rust implementation
- Add import for pyo3
- Add implementation for Rust function
- Attach Rust function to module-exporting-stuff

6. Replace python implementation with call to Rust

6.5 Unexpected jiggery-pokery
```
mv ./fib/fib.cpython-311-darwin.so ./fib.so
```

7. Results
```
❯ time python3 fibonacci.py 25
25: 75025
python3 fibonacci.py 25  0.02s user 0.02s system 10% cpu 0.377 total
❯ time python3 fibonacci.py 30
30: 832040
python3 fibonacci.py 30  0.02s user 0.02s system 15% cpu 0.268 total
❯ time python3 fibonacci.py 35
35: 9227465
python3 fibonacci.py 35  0.04s user 0.02s system 20% cpu 0.277 total
```

