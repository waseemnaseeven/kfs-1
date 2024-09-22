# How do we made it

## Useful cmds
- `rustup component list`
- `rustup show`


## Rust Binary
1- Creating our own operating system is to create a Rust executable that does not link the std library. So `cargo new <name> --bin` cuz we want to create an exe instead of a library. We disable in our main.rs the std by adding `#![no_std]`.

2- If we compile now, the compiler require 'panic handler' and 'eh_personality'.
    - `panic` macro take action when an array past the end for example
    - `eh_personality` item is used for implementing stack unwinding : permet de prevenir les leaks.

in cargo.toml we will abort panic :

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```
3- Overwriting the entrypoint by putting : #![no_main]

4- #[no_mangle] without this attribute, Rust compiler attribute a unique name generated like '_Wuoihg548_huge', instead we ensure that Rust compiler really outputs a function with the name _start.

5- Avoid linker pb with the following cmd : `cargo rustc -- -C link-arg=nostartfiles 
it tell the linker that it should not link the C startup routine.

## Minimal Kernel
1- When you turn on a computer, it begins executing firmware code that is stored in motherboard ROM. This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware. Afterwards, it looks for a bootable disk and starts booting the operating system kernel. On x86, there are two firmware standards: the “Basic Input/Output System“ (BIOS) and the newer “Unified Extensible Firmware Interface” (UEFI). 

2- We create a Disk Image that prints Hello World!. For that we need `nightly compiler` and not the stable or the beta cuz we need some experimental features.

3- x86_64-kfs-1.json file : Defining our own architecture, our own target system

4- Config i've made ON my computer not on the code 
- .cargo/config.toml created
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]

[build]
target = "x86_64-kfs-1.json"
```

5- The code that print hello_world, see the code on main.rs (might stink your eye but there is ChapGPT comments, no worries). EDIT: i change the code in main.rs so here is the code:
```rust
// Writing directly in VGA buffer "Hello, World!"
// 'b' prefix = byte string, every character mean an ASCII character
static HELLO: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 0xb8000 : adress of VGA buffer, espace mem dedie au display text sur system x_86
    // *mut u8 : mutable pointer to an byte (octet)
    let vga_buffer = 0xb8000 as *mut u8;

    // enumerate() give index i and a reference to byte for every character
    for (i, &byte) in HELLO.iter().enumerate() {
        // unsecure access to the memory
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
```

6 - Creating a bootimage : 
- `cargo add bootloader@0.9`
- `rustup component add llvm-tools-preview`

### WARNING
au moment de l'installation du bootloader, supprimer le fichier config.toml du .cargo puis reecrire ce fichier apres avoir executer la cmd : 
- `cargo install bootimage`
- `cargo bootimage`


```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]

[build]
target = "x86_64-kfs-1.json"

[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

### VGA buffer

1- Representing color using c-Like Enum, add TextBuffer as struct where we define the buffer height and width. Volatile dependencies tells the compiler that the write has side effects and should not be optimized away (cuz Rust compiler always try to optimize).

2- en francais pcq un peu complique. Ensuite, creation d'une struct Writer qui sera la structure fonctionnelle d'ecriture sur le buffer. La macro lazy static permet d'init des variables statique, et elles sont statiques seulement lorsqu'elles y accedent pour la premiere fois. 
`static ref` est une ref statique immuable mais le contenu peut etre mutable grace au Mutex. Mutex<Writer> est une structure de synchronisation qui assure que seul un thread peut acceder a la ressource partagee. Ca garantit l'acces sur au Writer dans un env multitache ou multithread. 

3- println function using macro_rules!

4- Panic functions

### Testing

1- we split the cargo run as the result we should have during the project AND cargo test as every functionalities we should test.

2- integration test
