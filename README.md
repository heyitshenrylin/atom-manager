# Atom Manager
### A Custom Atom Installs Manager Using Rust
###### Author: [Henry Lin](https://github.com/heyitshenrylin)

## So What Is It?

After being shown how to make additional atom installs with different packages
for each language, I had found it slightly annoying to go through and find which
atom I needed to use, and so I had thought to have one unified atom to click on
in my taskbar and then just type in a number to quickly open the atom I wanted.

## How To Use It?

Create a shortcut to the executable file to open up the command line and simply
type in the number to open up your specified atom install.

If it's the first time running you will need to change your PATH settings within
the code and then re-compile by navigating to the `src` folder and then typing
`rustc main.rs`

Please also take a look at the dependencies in case you don't have them

## Current Version

---
### v0.1.0 - Feb 28 2019
---

Basic functionality, running the shortcut to the executable file prompts the
user in a command line to enter a number for which atom install to run.

v0.1.0 does not include my installs for atom, running the .exe file will create
a brand new atom install in the `atom-installs` folder.

## Dependencies

The atom manager only works on Windows based systems due to it's need for batch
scripts.

The main language used is Rust, you will need it's compiler `Rustc` to compile
the `main.rs` file.

You can install Rust [here](https://www.rust-lang.org/tools/install)

## Known Issues

* Redirecting PATH is still tedious in comparison to the batch files

* Github desktop is unhappy with committing too many files

* Having to press 'Enter' after making the choice is not very smooth
 >Figuring out how the input stream works in Rust would be able to fix this

* Revising the atom-manager and my actual installs folder is not streamlined

---

**Special thanks to [Imswebra](https://github.com/imswebra "Github Profile") for
his atom setup for c++ and python with the batch scripts**
