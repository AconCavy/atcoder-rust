@echo off

set PROJECT=%~1

if "%PROJECT%"=="" (
  echo The command requires a project name.
  exit /b
)

set OUTPUT=%CD%\Other
set CONTEST_TYPE=%PROJECT:~0,3%
echo %CONTEST_TYPE% | findstr /r "a.c"

if %ERRORLEVEL%==0 (
  set OUTPUT=%CD%\%CONTEST_TYPE%
)

mkdir %OUTPUT%
mkdir %OUTPUT%\%PROJECT%
mkdir %OUTPUT%\%PROJECT%\src
mkdir %OUTPUT%\%PROJECT%\src\bin

setlocal enabledelayedexpansion

for %%p in (a b c d e f) do (
  set OUTPUT_FILE=%OUTPUT%\%PROJECT%\src\bin\%%p.rs

  echo #^^!^[allow^(dead_code^)^]>> !OUTPUT_FILE!
  echo #^^!^[allow^(unused_imports^)^]>> !OUTPUT_FILE!
  echo;>> !OUTPUT_FILE!
  echo use itertools::Itertools;>> !OUTPUT_FILE!
  echo use proconio::marker::{Bytes, Chars, Usize1};>> !OUTPUT_FILE!
  echo use proconio::*;>> !OUTPUT_FILE!
  echo use std::cmp::*;>> !OUTPUT_FILE!
  echo use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};>> !OUTPUT_FILE!
  echo use std::io;>> !OUTPUT_FILE!
  echo use std::mem::*;>> !OUTPUT_FILE!
  echo;>> !OUTPUT_FILE!
  echo #^[fastout^]>> !OUTPUT_FILE!
  echo fn main^(^) ^{^}>> !OUTPUT_FILE!
)

endlocal

set CARGO_FILE=%OUTPUT%\%PROJECT%\Cargo.toml

echo [package]>> %CARGO_FILE%
echo name = "%PROJECT%">> %CARGO_FILE%
echo version = "0.1.0">> %CARGO_FILE%
echo edition = "2018">> %CARGO_FILE%
echo;>> %CARGO_FILE%
echo # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html>> %CARGO_FILE%
echo;>> %CARGO_FILE%
echo [dependencies]>> %CARGO_FILE%
echo num = "=0.2.1">> %CARGO_FILE%
echo num-bigint = "=0.2.6">> %CARGO_FILE%
echo num-complex = "=0.2.4">> %CARGO_FILE%
echo num-integer = "=0.1.42">> %CARGO_FILE%
echo num-iter = "=0.1.40">> %CARGO_FILE%
echo num-rational = "=0.2.4">> %CARGO_FILE%
echo num-traits = "=0.2.11">> %CARGO_FILE%
echo num-derive = "=0.3.0">> %CARGO_FILE%
echo ndarray = "=0.13.0">> %CARGO_FILE%
echo nalgebra = "=0.20.0">> %CARGO_FILE%
echo alga = "=0.9.3">> %CARGO_FILE%
echo libm = "=0.2.1">> %CARGO_FILE%
echo rand = { version = "=0.7.3", features = ["small_rng"] }>> %CARGO_FILE%
echo getrandom = "=0.1.14">> %CARGO_FILE%
echo rand_chacha = "=0.2.2">> %CARGO_FILE%
echo rand_core = "=0.5.1">> %CARGO_FILE%
echo rand_hc = "=0.2.0">> %CARGO_FILE%
echo rand_pcg = "=0.2.1">> %CARGO_FILE%
echo rand_distr = "=0.2.2">> %CARGO_FILE%
echo petgraph = "=0.5.0">> %CARGO_FILE%
echo indexmap = "=1.3.2">> %CARGO_FILE%
echo regex = "=1.3.6">> %CARGO_FILE%
echo lazy_static = "=1.4.0">> %CARGO_FILE%
echo ordered-float = "=1.0.2">> %CARGO_FILE%
echo ascii = "=1.0.0">> %CARGO_FILE%
echo permutohedron = "=0.2.4">> %CARGO_FILE%
echo superslice = "=1.0.0">> %CARGO_FILE%
echo itertools = "=0.9.0">> %CARGO_FILE%
echo itertools-num = "=0.1.3">> %CARGO_FILE%
echo maplit = "=1.0.2">> %CARGO_FILE%
echo either = "=1.5.3">> %CARGO_FILE%
echo im-rc = "=14.3.0">> %CARGO_FILE%
echo fixedbitset = "=0.2.0">> %CARGO_FILE%
echo bitset-fixed = "=0.1.0">> %CARGO_FILE%
echo proconio = { version = "=0.3.6", features = ["derive"] }>> %CARGO_FILE%
echo text_io = "=0.1.8">> %CARGO_FILE%
echo whiteread = "=0.5.0">> %CARGO_FILE%
echo rustc-hash = "=1.1.0">> %CARGO_FILE%
echo smallvec = "=1.2.0">> %CARGO_FILE%
