# YLab Edge

In a sister project we have introduced the [YLab](https://github.com/schmettow/ylab) for building interactive sensor recording devices. 
Using CircuitPython, developing sensors for everyday research never was easier.

The purpose of *YLab Edge* is to follow YLab in spirit, but improve on what Ylab lacks the most, and that is: speed! 
Highest achieved readings with YLab are in the range of 250 SPS, which is enough for many applications, 
but is insufficient for large sensor arrays with high sample rates, e.g. motion capture or EEG.
The solution is to re-implement the YLab API in the systems programming language [Rust](https://www.rust-lang.org/). 

**YLab Edge Pro** is the version to use for **STM32** microcontrollers. 
Currently, Pro is also the most powerful version in terms of channels and throughput.
However, it is not interactive, like YLab Go.

# Current status

Currenty, the following devices are implemented using [Embassy]: https://embassy.dev/. All devices are running in their own async task.

+ LED
+ on-board ADC channels

A test system with 

+ 16 analog channels in two banks @ 600 Hz

## Installing

All code in this crate is currently developed for an STM Nucleo-144 F446 board.

To install the latest version of YLab Edge Pro:

+ Install Rust and Cargo on your system
+ clone this repository (e.g. in VSC)
+ run cargo update
+ connect an STM32 Nucleo board via USB
+ open a terminal in the ylab-edge folder and type:

```console
$ cargo run --bin ylab_dg
```
If you get an error about not being able to find `probe-rs`, try:

```console
$ cargo install probe-rs
```
then try repeating the `cargo run` command above.



`SPDX-License-Identifier: Apache-2.0 OR MIT`

