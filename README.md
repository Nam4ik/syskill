<h1 align="center"> *NIX suicide kit </h1>
<p align="center">
  <a href="https://github.com/Nam4ik/syskill/actions/workflows/rust.yml">
    <img alt="CI" src="https://img.shields.io/github/actions/workflow/status/Nam4ik/syskill/rust.yml?branch=main&label=CI">
  </a>
  <a href="LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/Nam4ik/syskill">
  </a>
  <img alt="C" src="https://img.shields.io/badge/C-00599C?logo=c&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white">
  <a href="https://github.com/Nam4ik/syskill/issues">
    <img alt="Issues" src="https://img.shields.io/github/issues/Nam4ik/syskill">
  </a>
  <a href="https://github.com/Nam4ik/syskill/stargazers">
    <img alt="Stars" src="https://img.shields.io/github/stars/Nam4ik/syskill">
  </a>
</p>

A utility that will help you beautifully deal with your *NIX system. 

> [!WARNING] 
> DONT USE THIS ON PRODUCTION OR REAL PC, SOME FUNCTIONS MAY CAUSE YOUR SYSTEM TO BECOME COMPLETELY UNSTABLE.
> But when calling a function located in `src/non-critical/`, all the consequences will go away after a reboot.

This package provides a CLI utility with a set of functions for mocking your operating system, both critical and non-critical. From the already familiar fork bombs `:(){ :|:& };:` or something more critical like `sudo rm -rf /* --no-preserve-root` before rendering graphic artifacts by interfering with the frame buffer.

I also neglected optimization in this project. Seriously, what kind of optimization and code cleanliness are you talking about in a project that's literally designed to consolidate all possible ways to abuse or kill your system into one?

Most of the functions have been rewritten to suit the features of different UNIX-like systems (FreeBSD, OpenBSD, linux, etc...).
In the future, an addition to the project is planned in the form of similar abilities but for `win32`
