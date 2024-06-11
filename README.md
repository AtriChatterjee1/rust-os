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
qemu-system-x86_64 -drive format=raw,file=[file_location]
```
Replace ``` file_location``` with the location of your location of bootimage generated from ``` cargo bootimage ```
An example would look like : 

```
qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-rust_os.bin
```

This would print out Hello World on your Qemu emulator display. 

Thanks for checking out this repository, if you'd find this insightful, do star the repo!
