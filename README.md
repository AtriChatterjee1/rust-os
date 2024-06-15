# Creating an Operating System on Rust

This project aims to build a minimalistic Operating System on [Rust programming language](https://www.rust-lang.org/).

Why Rust? 
- Its low-level with memory safety implemented,
- Extremely fast (low latency)

Currently has only BIOS support, will add UEFI support progressively!

## How to Replicate 

Clone the repo, and open terminal on the same directory 

```
rustup component add llvm-tools-preview
```
To create the bootimage, use the following snippet: 

```
cargo install bootimage
cargo bootimage
```
This would give you an output of the form : 

![image](https://github.com/AtriChatterjee1/rust-os/assets/112633183/942dd7a5-7b91-4198-ac5f-5fa47031d045)

Now run the following command, to print hello-world.

```
cargo run
```

This would print out Hello World on your Qemu emulator display. 

To run tests run 
```
cargo test 
```

or if you'd like to run a specific test file use this command 

```
cargo test --test [file_name]
cargo test --test should_panic     // example
```
Be careful while running should_panic, as it gives sucess when the test panicks, and fails when all tests run fine!

Thanks for checking out this repository, if you'd find this insightful, do star the repo!
