<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools/` instead. 🚨🚨 -->

 <a href="http://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.png" />
 </a>

This repository lists **dynamic analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings and user comments for each tool.

## What is Dynamic Analysis?

> Dynamic program analysis is the analysis of computer software that is performed by executing programs on a real or virtual processor. — [Wikipedia](https://en.wikipedia.org/wiki/Dynamic_program_analysis)
## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
  <tr>
    <td><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepcode.png" /></a></td>
    <td><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codescene.svg" /></a></td>
    <td><a href="https://r2c.dev/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/r2c.svg" /></a></td>
  </tr>
</table>

If you also want to support this project, head over to our [Github sponsors page](https://github.com/sponsors/analysis-tools-dev).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-static-analysis](https://github.com/mre/awesome-static-analysis).

## Table of Contents

#### [Programming Languages](#programming-languages-1)

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    <li><a href="#dotnet">.NET</a></li>
    <li><a href="#c">C</a></li>
    <li><a href="#cpp">C++</a></li>
    <li><a href="#java">Java</a></li>
    <li><a href="#javascript">JavaScript</a></li>
    <li><a href="#python">Python</a></li>
    <li><a href="#rust">Rust</a></li>
    <li><a href="#vbasic">Visual Basic</a></li>
    </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

- [API](#api)
  - [Binaries](#binary)
  - [Bytecode/IR](#bytecode)
  - [Containers](#container)
  - [Security/DAST](#security)
  - [Web](#web)
  

---

## Programming Languages

<h2 id="dotnet">.NET</h2>

- [Microsoft IntelliTest](https://docs.microsoft.com/en-us/visualstudio/test/intellitest-manual/getting-started?view=vs-2019) - Generate a candidate suite of tests for your .NET code.
- [Pex and Moles](https://www.microsoft.com/en-us/research/project/pex-and-moles-isolation-and-white-box-unit-testing-for-net/) - Pex automatically generates test suites with high code coverage using automated white box analysis.


<h2 id="c">C</h2>

- [CHAP](https://github.com/vmware/chap) - Analyzes un-instrumented ELF core files for leaks, memory growth, and corruption. It helps explain memory growth, can identify some forms of corruption, and  supplements a debugger by giving the status of various memory locations.
- [KLEE](https://github.com/klee/klee) - Symbolic virtual machine built on top of the LLVM compiler infrastructure.
- [LDRA](https://ldra.com) :copyright: - A tool suite including dynamic analysis and test to various standards can ensure test coverage to 100% op-code, branch & decsion coverage.
- [LLVM/Clang Sanitizers](https://github.com/google/sanitizers) - <ul> <li><a href="https://github.com/google/sanitizers/wiki/AddressSanitizer">AddressSanitizer</a> - A memory error detector for C/C++</li> <li><a href="https://github.com/google/sanitizers/wiki/MemorySanitizer">MemorySanitizer</a> - A detector of uninitialized memory reads in C/C++ programs.</li> <li><a href="https://github.com/google/sanitizers/wiki/ThreadSanitizerCppManual">ThreadSanitizer</a> - A data race detector for C/C++</li> </ul>
- [tis-interpreter](https://github.com/TrustInSoft/tis-interpreter) :warning: - An interpreter for finding subtle bugs in programs written in standard C.
- [Valgrind](https://valgrind.org/) - An instrumentation framework for building dynamic analysis tools.


<h2 id="cpp">C++</h2>

- [CHAP](https://github.com/vmware/chap) - Analyzes un-instrumented ELF core files for leaks, memory growth, and corruption. It helps explain memory growth, can identify some forms of corruption, and  supplements a debugger by giving the status of various memory locations.
- [KLEE](https://github.com/klee/klee) - Symbolic virtual machine built on top of the LLVM compiler infrastructure.
- [LDRA](https://ldra.com) :copyright: - A tool suite including dynamic analysis and test to various standards can ensure test coverage to 100% op-code, branch & decsion coverage.
- [LLVM/Clang Sanitizers](https://github.com/google/sanitizers) - <ul> <li><a href="https://github.com/google/sanitizers/wiki/AddressSanitizer">AddressSanitizer</a> - A memory error detector for C/C++</li> <li><a href="https://github.com/google/sanitizers/wiki/MemorySanitizer">MemorySanitizer</a> - A detector of uninitialized memory reads in C/C++ programs.</li> <li><a href="https://github.com/google/sanitizers/wiki/ThreadSanitizerCppManual">ThreadSanitizer</a> - A data race detector for C/C++</li> </ul>
- [tis-interpreter](https://github.com/TrustInSoft/tis-interpreter) :warning: - An interpreter for finding subtle bugs in programs written in standard C.
- [Valgrind](https://valgrind.org/) - An instrumentation framework for building dynamic analysis tools.


<h2 id="java">Java</h2>

- [Java PathFinder](https://github.com/javapathfinder/jpf-core) - An extensible software model checking framework for Java bytecode programs.
- [Parasoft Jtest](https://www.parasoft.com/products/jtest) :copyright: - Jtest is an automated Java software testing and static analysis product that is made by Parasoft. The product includes technology for Data-flow analysis Unit test-case generation and execution, static analysis, regression testing, code coverage, and runtime error detection.


<h2 id="javascript">JavaScript</h2>

- [Iroh.js](https://github.com/maierfelix/Iroh) :warning: - A dynamic code analysis tool for JavaScript. Iroh allows to record your code flow in realtime, intercept runtime informations and manipulate program behaviour on the fly.
- [Jalangi2](https://github.com/Samsung/jalangi2) - Jalangi2 is a popular framework for writing dynamic analyses for JavaScript.


<h2 id="python">Python</h2>

- [CrossHair](https://github.com/pschanely/CrossHair) - Symbolic execution engine for testing Python contracts.
- [icontract](https://github.com/Parquery/icontract) - Design-by-contract library supporting behavioral subtyping
There is also a wider tooling around the icontract library such as  a linter (pyicontract-lint) and a plug-in for Sphinx (sphinx-icontract).
- [Scalene](https://github.com/emeryberger/scalene) - A high-performance, high-precision CPU and memory profiler for Python
- [typo](https://github.com/aldanor/typo) :warning: - Runtime Type Checking for Python 3.


<h2 id="rust">Rust</h2>

- [MIRI](https://github.com/rust-lang/miri) - An interpreter for Rust's mid-level intermediate representation, which can detect certain classes of undefined behavior like out-of-bounds memory accesses and use-after-free.
- [puffin](https://github.com/EmbarkStudios/puffin) - Instrumentation profiler for Rust.
- [stuck](https://github.com/jonhoo/stuck) :warning: - provides a visualization for quickly identifying common bottlenecks in running, asynchronous, and concurrent applications.


<h2 id="vbasic">Visual Basic</h2>

- [VB Watch](https://www.aivosto.com/vbwatch.html) :copyright: - Profiler, Protector and Debugger for VB6. Profiler measures performance and test coverage. Protector implements robust error handling. Debugger helps monitor your executables.


## Multiple languages

- [Code Pulse](http://code-pulse.com/) - Code Pulse is a free real-time code coverage tool for penetration testing activities by OWASP and Code Dx ([GitHub](https://github.com/codedx/codepulse)).
- [Enlightn](https://www.laravel-enlightn.com/) - A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.
- [Gcov](https://gcc.gnu.org/onlinedocs/gcc/Gcov.html) - GNU source code coverage program. Code coverage tool and profiling tool which is part of the GCC. Supports C, C++, Fortran.
- [WhiteHat Sentinel Dynamic](https://www.whitehatsec.com/products/dynamic-application-security-testing/) :copyright: - Part of the WhiteHat Application Security Platform. Dynamic application security scanner that covers the OWASP Top 10.


## Other



<h2 id="api">API</h2>

- [Smartbear](https://smartbear.com/) :copyright: - Test automation and performance testing platform


<h2 id="binary">Binaries</h2>

- [angr](https://github.com/angr/angr) - Platform agnostic binary analysis framework from UCSB.
- [BOLT](https://github.com/facebookincubator/BOLT) - Binary Optimization and Layout Tool - A linux command-line utility used for optimizing performance of binaries  with profile guided permutation of linking to improve cache efficiency
- [Dr. Memory](https://drmemory.org/) - Dr. Memory is a memory monitoring tool capable of identifying memory-related programming errors ([Github](https://github.com/DynamoRIO/drmemory)).
- [DynamoRIO](http://www.dynamorio.org/) - Is a runtime code manipulation system that supports code transformations on any part of a program, while it executes.
- [llvm-propeller](https://github.com/google/llvm-propeller) - Profile guided hot/cold function splitting to improve cache efficiency. An alternative to BOLT by Facebook
- [Pin Tools](https://software.intel.com/en-us/articles/pin-a-dynamic-binary-instrumentation-tool) - A dynamic binary instrumentation tool and a platform for creating analysis tools.
- [TRITON](https://triton.quarkslab.com/) - Dynamic Binary Analysis for x86 binaries.


<h2 id="bytecode">Bytecode/IR</h2>

- [souper](https://github.com/google/souper) - optimize LLVM IR with SMT solvers


<h2 id="container">Containers</h2>

- [cadvisor](https://github.com/google/cadvisor) - Analyzes resource usage and performance characteristics of running containers.


<h2 id="security">Security/DAST</h2>

- [AppScan Standard](https://www.hcltechsw.com/products/appscan) :copyright: - HCL's AppScan is a dynamic application security testing suite ([previously by IBM](https://newsroom.ibm.com/2018-12-06-HCL-Technologies-to-Acquire-Select-IBM-Software-Products-for-1-8B)).
- [Minded Security BlueClosure](https://www.blueclosure.com) :copyright: - Dynamic web application security scanner. It uses dynamic data tainting in order to understand if a DOM XSS is exploitable and uses the browser JavaScript engine for understanding the code.
- [WebScanner](https://www.defensecode.com/web-security-scanner-dast/) :copyright: - WebScanner is a DAST solution for comprehensive security audits of active web applications.


<h2 id="web">Web</h2>

- [Smartbear](https://smartbear.com/) :copyright: - Test automation and performance testing platform


## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.

Title image [Designed by Freepik](http://www.freepik.com).