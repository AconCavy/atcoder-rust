@echo off

set PROJECT=%~1
set TASK=%~2

set IS_HELP=false
if "%PROJECT%"=="" set IS_HELP=true
if "%PROJECT%"=="help" set IS_HELP=true

if %IS_HELP%==true (
  echo Usage: atcoder.cmd [project-name]                Create a new project.
  echo Usage: atcoder.cmd [project-name] [task-name]    Create a new task to the project.
  exit /b
)

set OUTPUT=%CD%\Other
set CONTEST_TYPE=%PROJECT:~0,3%
echo %CONTEST_TYPE% | findstr /r "a[bghr]c" > nul

if %ERRORLEVEL%==0 (
  set OUTPUT=%CD%\%CONTEST_TYPE%
)

if not exist %OUTPUT% (
  echo Create %OUTPUT%.
  mkdir %OUTPUT%
)

set PROJECT_PATH=%OUTPUT%\%PROJECT%

if not exist %PROJECT_PATH% (
  echo Create %PROJECT_PATH%.
  mkdir %PROJECT_PATH%
  echo Create %PROJECT_PATH%\src.
  mkdir %PROJECT_PATH%\src
  echo Create %PROJECT_PATH%\src\bin.
  mkdir %PROJECT_PATH%\src\bin
)

if not exist %PROJECT_PATH%\Cargo.toml (
  call :cargofile %PROJECT_PATH%
)

if "%TASK%"=="" (
  setlocal enabledelayedexpansion
  for %%p in (a b c d e f) do (
    call :taskfile !PROJECT_PATH! %%p
  )
  endlocal
) else (
  call :taskfile %PROJECT_PATH% %TASK%
)

exit /b

:taskfile
  setlocal
  set PROJECT_PATH=%~1
  set TASK=%~2
  set OUTPUT_FILE=%PROJECT_PATH%\src\bin\%TASK%.rs

  if exist %OUTPUT_FILE% (
    echo %OUTPUT_FILE% is already exist. Skip creating the file.
    exit /b
  )

  echo Create %OUTPUT_FILE%.

  echo #^^![allow(dead_code)]>> %OUTPUT_FILE%
  echo #^^![allow(unused_imports)]>> %OUTPUT_FILE%
  echo;>> %OUTPUT_FILE%
  echo use itertools::Itertools;>> %OUTPUT_FILE%
  echo use proconio::marker::{Bytes, Chars, Usize1};>> %OUTPUT_FILE%
  echo use proconio::*;>> %OUTPUT_FILE%
  echo use std::cmp::*;>> %OUTPUT_FILE%
  echo use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};>> %OUTPUT_FILE%
  echo use std::io;>> %OUTPUT_FILE%
  echo use std::mem::*;>> %OUTPUT_FILE%
  echo;>> %OUTPUT_FILE%
  echo #[fastout]>> %OUTPUT_FILE%
  echo fn main() {}>> %OUTPUT_FILE%

  endlocal
  exit /b

:cargofile
  setlocal
  set PROJECT_PATH=%~1
  set OUTPUT_FILE=%PROJECT_PATH%\Cargo.toml

  if exist %OUTPUT_FILE% (
    echo %OUTPUT_FILE% is already exist. Skip creating the file.
    exit /b
  )

  echo Create %OUTPUT_FILE%.

  echo [package]>> %OUTPUT_FILE%
  echo name = "%PROJECT%">> %OUTPUT_FILE%
  echo version = "0.1.0">> %OUTPUT_FILE%
  echo edition = "2018">> %OUTPUT_FILE%
  echo;>> %OUTPUT_FILE%
  echo # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html>> %OUTPUT_FILE%
  echo;>> %OUTPUT_FILE%
  echo [dependencies]>> %OUTPUT_FILE%
  echo num = "=0.2.1">> %OUTPUT_FILE%
  echo num-bigint = "=0.2.6">> %OUTPUT_FILE%
  echo num-complex = "=0.2.4">> %OUTPUT_FILE%
  echo num-integer = "=0.1.42">> %OUTPUT_FILE%
  echo num-iter = "=0.1.40">> %OUTPUT_FILE%
  echo num-rational = "=0.2.4">> %OUTPUT_FILE%
  echo num-traits = "=0.2.11">> %OUTPUT_FILE%
  echo num-derive = "=0.3.0">> %OUTPUT_FILE%
  echo ndarray = "=0.13.0">> %OUTPUT_FILE%
  echo nalgebra = "=0.20.0">> %OUTPUT_FILE%
  echo alga = "=0.9.3">> %OUTPUT_FILE%
  echo libm = "=0.2.1">> %OUTPUT_FILE%
  echo rand = { version = "=0.7.3", features = ["small_rng"] }>> %OUTPUT_FILE%
  echo getrandom = "=0.1.14">> %OUTPUT_FILE%
  echo rand_chacha = "=0.2.2">> %OUTPUT_FILE%
  echo rand_core = "=0.5.1">> %OUTPUT_FILE%
  echo rand_hc = "=0.2.0">> %OUTPUT_FILE%
  echo rand_pcg = "=0.2.1">> %OUTPUT_FILE%
  echo rand_distr = "=0.2.2">> %OUTPUT_FILE%
  echo petgraph = "=0.5.0">> %OUTPUT_FILE%
  echo indexmap = "=1.3.2">> %OUTPUT_FILE%
  echo regex = "=1.3.6">> %OUTPUT_FILE%
  echo lazy_static = "=1.4.0">> %OUTPUT_FILE%
  echo ordered-float = "=1.0.2">> %OUTPUT_FILE%
  echo ascii = "=1.0.0">> %OUTPUT_FILE%
  echo permutohedron = "=0.2.4">> %OUTPUT_FILE%
  echo superslice = "=1.0.0">> %OUTPUT_FILE%
  echo itertools = "=0.9.0">> %OUTPUT_FILE%
  echo itertools-num = "=0.1.3">> %OUTPUT_FILE%
  echo maplit = "=1.0.2">> %OUTPUT_FILE%
  echo either = "=1.5.3">> %OUTPUT_FILE%
  echo im-rc = "=14.3.0">> %OUTPUT_FILE%
  echo fixedbitset = "=0.2.0">> %OUTPUT_FILE%
  echo bitset-fixed = "=0.1.0">> %OUTPUT_FILE%
  echo proconio = { version = "=0.3.6", features = ["derive"] }>> %OUTPUT_FILE%
  echo text_io = "=0.1.8">> %OUTPUT_FILE%
  echo whiteread = "=0.5.0">> %OUTPUT_FILE%
  echo rustc-hash = "=1.1.0">> %OUTPUT_FILE%
  echo smallvec = "=1.2.0">> %OUTPUT_FILE%

  endlocal
  exit /b
