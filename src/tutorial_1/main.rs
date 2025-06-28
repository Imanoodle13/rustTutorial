/* Intro to Rust Programming */
fn main() {
    println!("Hello, world!");
}

/*
COMPILE AND RUN YOUR RUST CODE:
1. Set up the following in Cargo.tml:
    [[bin]]
    name = "{program_name}"
    path = "src/{program_name}.rs"
2. `cd target`
3. `cd debug`
4. `cargo run`:
    PS C:\{directories}\target\debug> cargo run
    Compiling rustTutorial v0.1.0 (C:\{directories})
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
    Running `.\{program_name}.exe`
    Hello, world!

Additionally:
`cargo check`:
-   It checks if the code can compile without actually compiling it.
-   Not applicable at this scale but if you had a project that
    utilizes Cargo, the outputs/errors will be listed.
*/