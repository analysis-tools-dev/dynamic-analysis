 <!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

 <a href="https://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.png" />
 </a>

This repository lists **dynamic analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings and user comments for each tool.

## What is Dynamic Analysis?

> Dynamic program analysis is the analysis of computer software that is performed by executing programs on a real or virtual processor. — [Wikipedia](https://en.wikipedia.org/wiki/Dynamic_program_analysis)

## Sponsors

This project would not be possible without the generous support of our sponsors.

<ul>
  <li><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepcode.svg" /></a></li>
  <li><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codescene.png" /></a></li>
</ul>

If you also want to support this project, head over to our [Github sponsors page](https://github.com/sponsors/analysis-tools-dev).

## Meaning of Symbols:  

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 6 months, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-static-analysis](https://github.com/mre/awesome-static-analysis).

## Binary

* [angr](https://github.com/angr/angr) - Platform agnostic binary analysis framework from UCSB.
* [TRITON](https://github.com/JonathanSalwan/Triton) - Dynamic Binary Analysis for x86 binaries.
* [DynamoRIO](http://www.dynamorio.org/) - is a runtime code manipulation system that supports code transformations on any part of a program, while it executes.
* [Pin Tools](https://software.intel.com/en-us/articles/pin-a-dynamic-binary-instrumentation-tool) - A dynamic binary instrumentation tool and a platform for creating analysis tools.

## C/C++

* [KLEE](https://github.com/klee/klee) - Symbolic virtual machine built on top of the LLVM compiler infrastructure.
* [tis-interpreter](https://github.com/TrustInSoft/tis-interpreter) - An interpreter for finding subtle bugs in programs written in standard C
* [Valgrind](http://valgrind.org/) - An instrumentation framework for building dynamic analysis tools
* [LDRA](https://ldra.com) :copyright: - a tool suite incuding dynamic analysis and test to various standards can ensure test coverage to 100% op-code, branch & decsion coverage.
* [LLVM/Clang Sanitizers](https://github.com/google/sanitizers)
    - [AddressSanitizer](https://github.com/google/sanitizers/wiki/AddressSanitizer) - A memory error detector for C/C++
    - [MemorySanitizer](https://github.com/google/sanitizers/wiki/MemorySanitizer) - A detector of uninitialized memory reads in C/C++ programs.
    - [ThreadSanitizer](https://github.com/google/sanitizers/wiki/ThreadSanitizerCppManual) - A data race detector for C/C++

## Java

* [Java PathFinder](https://github.com/javapathfinder/jpf-core) - An extensible software model checking framework for Java bytecode programs.
* [Parasoft Jtest](https://www.parasoft.com/products/jtest) :copyright: - Jtest is an automated Java software testing and static analysis product that is made by Parasoft. The product includes technology for Data-flow analysis Unit test-case generation and execution, static analysis, regression testing, code coverage, and runtime error detection.

## Python

* [typo](https://github.com/aldanor/typo) - Runtime Type Checking for Python 3

## .NET

* [Microsoft IntelliTest](https://docs.microsoft.com/en-us/visualstudio/test/intellitest-manual/getting-started?view=vs-2019) - Generate a candidate suite of tests for your .NET code.
* [Pex and Moles](https://www.microsoft.com/en-us/research/project/pex-and-moles-isolation-and-white-box-unit-testing-for-net/) - Pex automatically generates test suites with high code coverage using automated white box analysis.

## Rust

* [MIRI](https://github.com/rust-lang/miri) -  An interpreter for Rust's mid-level intermediate representation, which can detect certain classes of undefined behavior like out-of-bounds memory accesses and use-after-free.
* [stuck](https://github.com/jonhoo/stuck) - provides a visualization for quickly identifying common bottlenecks in running, asynchronous, and concurrent applications.

## Visual Basic

* [VB Watch](https://www.aivosto.com/vbwatch.html) :copyright: - Profiler, Protector and Debugger for VB6. Profiler measures performance and test coverage. Protector implements robust error handling. Debugger helps monitor your executables.

## Multiple languages

* [AppScan Standard](https://www.hcltechsw.com/wps/portal/products/appscan/home/!ut/p/z1/04_Sj9CPykssy0xPLMnMz0vMAfIjo8zi_QO8nQ0MnQ0C_F3MnA0CHX2dvYN9woxNvEz0w1EVWDgGuQAVeLpbBvu6Gxl4m-hHUaLfxJQ4_QY4gKMBifZjKojCb3y4fhSqFe6Bpk5AEwIMTNyMfYzdfczQFWAJIrwKQGFAyBUFuaGhoREGmZ7piooAwLgEZw!!/?1dmy&urile=wcm%3apath%3a/wps/wcm/connect/hcl+software+content/products/appscan/offerings/standard) :copyright: - HCL's AppScan is a dynamic application security testing suite ([previously by IBM](https://newsroom.ibm.com/2018-12-06-HCL-Technologies-to-Acquire-Select-IBM-Software-Products-for-1-8B)).
* [Code Pulse](http://code-pulse.com/) - Code Pulse is a free real-time code coverage tool for penetration testing activities by OWASP and Code Dx ([GitHub](https://github.com/codedx/codepulse)).
* [Gcov](https://gcc.gnu.org/onlinedocs/gcc/Gcov.html) - GNU source code coverage program. Code coverage tool and profiling tool which is part of the GCC. Supports C, C++, Fortran.
* [Minded Security BlueClosure](https://www.mindedsecurity.com/index.php/products/blueclosure) :copyright: - Dynamic web application security scanner. It uses dynamic data tainting in order to understand if a DOM XSS is exploitable and uses the browser JavaScript engine for understanding the code.
* [WhiteHat Sentinel Dynamic](https://www.whitehatsec.com/products/dynamic-application-security-testing/) :copyright: - Part of the WhiteHat Application Security Platform. Dynamic application security scanner that covers the OWASP Top 10.

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.

Logo [designed by Freepik](https://www.freepik.com/free-vector/programming-background-design_1033623.htm)

