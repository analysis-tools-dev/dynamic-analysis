<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit files in `data/tools/` instead. 🚨🚨 -->

<a href="https://analysis-tools.dev/">
  <img alt="Analysis Tools Website" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/redesign.svg" />
</a>

This repository lists **dynamic analysis tools** for finding problems in running programs. It includes profilers, fuzzers, runtime analysis tools, and security testing tools.
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

[![Website](https://img.shields.io/badge/Website-Online-2B5BAE)](https://analysis-tools.dev)
[![CI](https://github.com/analysis-tools-dev/dynamic-analysis/workflows/CI/badge.svg)](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/ci.yml)
[![Links](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/links.yml/badge.svg)](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/links.yml)

## Sponsors

Thank you to CodeRabbit for sponsoring this project, and to everyone who has supported it over the years.

<a href="https://coderabbit.ai">
  <img width="200px" alt="CodeRabbit" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/code-rabbit.svg" />
</a>

Support this project through [GitHub Sponsors](https://github.com/sponsors/analysis-tools-dev) or [Open Collective](https://opencollective.com/analysis-tools).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-static-analysis](https://github.com/mre/awesome-static-analysis).

## Table of Contents

#### [Programming Languages](#programming-languages-1)

| | | |
|---|---|---|
| [.NET](#dotnet) | [C](#c) | [C++](#cpp) |
| [Go](#go) | [Java](#java) | [JavaScript](#javascript) |
| [PHP](#php) | [Python](#python) | [Ruby](#ruby) |
| [Rust](#rust) | [SQL](#sql) | [Visual Basic](#vbasic) |
| [Zig](#zig) |

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

<details>
<summary>Show Other</summary>

| | | |
|---|---|---|
| [API](#api) | [Binaries](#binary) | [Bytecode/IR](#bytecode) |
| [Cloud](#cloud) | [Containers](#container) | [Laravel](#laravel) |
| [Security/DAST](#security) | [Web](#web) | [WebAssembly](#webassembly) |
| [XML](#xml) |

</details>

---

## Programming Languages

<a id="dotnet"></a>
<h2>.NET</h2>


- [Microsoft IntelliTest](https://docs.microsoft.com/en-us/visualstudio/test/intellitest-manual/getting-started?view=vs-2019) — Generate a candidate suite of tests for your .NET code.

- [Pex and Moles](https://www.microsoft.com/en-us/research/project/pex-and-moles-isolation-and-white-box-unit-testing-for-net/) — Pex automatically generates test suites with high code coverage using automated white box analysis.



<a id="c"></a>
<h2>C</h2>


- [KLEE](https://github.com/klee/klee) — Symbolic virtual machine built on top of the LLVM compiler infrastructure.

- [LDRA](https://ldra.com) :copyright: — A tool suite including dynamic analysis and test to various standards can ensure test coverage to 100% op-code, branch & decsion coverage.

- [LLVM/Clang Sanitizers](https://github.com/google/sanitizers) — <ul> <li><a href="https://github.com/google/sanitizers/wiki/AddressSanitizer">AddressSanitizer</a> - A memory error detector for C/C++</li> <li><a href="https://github.com/google/sanitizers/wiki/MemorySanitizer">MemorySanitizer</a> - A detector of uninitialized memory reads in C/C++ programs.</li> <li><a href="https://github.com/google/sanitizers/wiki/ThreadSanitizerCppManual">ThreadSanitizer</a> - A data race detector for C/C++</li> </ul>

- [Valgrind](https://valgrind.org/) — An instrumentation framework for building dynamic analysis tools.



<details>
<summary>Show Deprecated</summary>


- [CHAP](https://github.com/vmware/chap) :warning: — Analyzes un-instrumented ELF core files for leaks, memory growth, and corruption. It helps explain memory growth, can identify some forms of corruption, and  supplements a debugger by giving the status of various memory locations.


</details>



<a id="cpp"></a>
<h2>C++</h2>


- [KLEE](https://github.com/klee/klee) — Symbolic virtual machine built on top of the LLVM compiler infrastructure.

- [LDRA](https://ldra.com) :copyright: — A tool suite including dynamic analysis and test to various standards can ensure test coverage to 100% op-code, branch & decsion coverage.

- [LLVM/Clang Sanitizers](https://github.com/google/sanitizers) — <ul> <li><a href="https://github.com/google/sanitizers/wiki/AddressSanitizer">AddressSanitizer</a> - A memory error detector for C/C++</li> <li><a href="https://github.com/google/sanitizers/wiki/MemorySanitizer">MemorySanitizer</a> - A detector of uninitialized memory reads in C/C++ programs.</li> <li><a href="https://github.com/google/sanitizers/wiki/ThreadSanitizerCppManual">ThreadSanitizer</a> - A data race detector for C/C++</li> </ul>

- [Valgrind](https://valgrind.org/) — An instrumentation framework for building dynamic analysis tools.



<details>
<summary>Show Deprecated</summary>


- [CHAP](https://github.com/vmware/chap) :warning: — Analyzes un-instrumented ELF core files for leaks, memory growth, and corruption. It helps explain memory growth, can identify some forms of corruption, and  supplements a debugger by giving the status of various memory locations.


</details>



<a id="go"></a>
<h2>Go</h2>


- [statsviz](https://github.com/arl/statsviz) — Instant live visualization of your Go application runtime statistics in the browser. It plots heap usage, MSpans/MCaches, Object counts, Goroutines and GC/CPU fraction.



<a id="java"></a>
<h2>Java</h2>


- [Java PathFinder](https://github.com/javapathfinder/jpf-core) — An extensible software model checking framework for Java bytecode programs.

- [Parasoft Jtest](https://www.parasoft.com/products/jtest) :copyright: — Jtest is an automated Java software testing and static analysis product that is made by Parasoft. The product includes technology for Data-flow analysis Unit test-case generation and execution, static analysis, regression testing, code coverage, and runtime error detection.



<a id="javascript"></a>
<h2>JavaScript</h2>


- [Jalangi2](https://github.com/Samsung/jalangi2) — Jalangi2 is a popular framework for writing dynamic analyses for JavaScript.



<details>
<summary>Show Deprecated</summary>


- [Iroh.js](https://github.com/maierfelix/Iroh) :warning: — A dynamic code analysis tool for JavaScript. Iroh allows to record your code flow in realtime, intercept runtime informations and manipulate program behaviour on the fly.


</details>



<a id="php"></a>
<h2>PHP</h2>




<details>
<summary>Show Deprecated</summary>


- [Enlightn](https://github.com/enlightn/enlightn) :warning: — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.


</details>



<a id="python"></a>
<h2>Python</h2>


- [CrossHair](https://github.com/pschanely/CrossHair) — Symbolic execution engine for testing Python contracts.

- [DynaPyt](https://github.com/sola-st/DynaPyt) — DynaPyt is a framework for writing dynamic analyses for Python. The analyses can also modify runtime values to alter the execution.

- [icontract](https://github.com/Parquery/icontract) — Design-by-contract library supporting behavioral subtyping
There is also a wider tooling around the icontract library such as  a linter (pyicontract-lint) and a plug-in for Sphinx (sphinx-icontract).



<details>
<summary>Show Deprecated</summary>


- [Scalene](https://github.com/emeryberger/scalene) :warning: — A high-performance, high-precision CPU and memory profiler for Python



- [typo](https://github.com/aldanor/typo) :warning: — Runtime Type Checking for Python 3.


</details>



<a id="ruby"></a>
<h2>Ruby</h2>


- [suture](https://github.com/testdouble/suture) — A Ruby gem that helps you refactor your legacy code  by the result of some old behavior with a new version.



<a id="rust"></a>
<h2>Rust</h2>


- [cargo-careful](https://github.com/RalfJung/cargo-careful) — Execute Rust code carefully, with extra checking along the way. It builds the standard library with debug assertions.
Here are some of the checks this enables:
* `get_unchecked` in slices performs bounds checks * `copy`, `copy_nonoverlapping`, and `write_bytes` check that pointers are aligned and non-null and (if applicable) non-overlapping `{NonNull,NonZero*,...}::new_unchecked` check that the value is valid * plenty of internal consistency checks in the collection types * mem::zeroed and the deprecated mem::uninitialized panic if the type does not allow that kind of initialization

- [hyperfine](https://github.com/sharkdp/hyperfine) — A command-line benchmarking tool It features statistical analysis across multiple runs, support for arbitrary shell commands, constant feedback about the benchmark progress and current estimates, warmup runs, a simple and expressive syntax, and more.

- [loom](https://github.com/tokio-rs/loom) — Concurrency permutation testing tool for Rust.  It runs a test many times, permuting the possible concurrent executions of that test.

- [MIRI](https://github.com/rust-lang/miri) — An interpreter for Rust's mid-level intermediate representation, which can detect certain classes of undefined behavior like out-of-bounds memory accesses and use-after-free.

- [puffin](https://github.com/EmbarkStudios/puffin) — Instrumentation profiler for Rust.



<details>
<summary>Show Deprecated</summary>


- [rust-san](https://github.com/japaric/rust-san) :warning: — How-to sanitize your Rust code with built-in Rust dynamic analyzers



- [stuck](https://github.com/jonhoo/stuck) :warning: — provides a visualization for quickly identifying common bottlenecks in running, asynchronous, and concurrent applications.


</details>



<a id="sql"></a>
<h2>SQL</h2>


- [WhiteHat Sentinel Dynamic](https://www.synopsys.com/software-integrity/security-testing/dast.html) :copyright: — Part of the WhiteHat Application Security Platform. Dynamic application security scanner that covers the OWASP Top 10.



<a id="vbasic"></a>
<h2>Visual Basic</h2>


- [VB Watch](https://www.aivosto.com/vbwatch.html) :copyright: — Profiler, Protector and Debugger for VB6. Profiler measures performance and test coverage. Protector implements robust error handling. Debugger helps monitor your executables.



<a id="zig"></a>
<h2>Zig</h2>


- [poop](https://github.com/andrewrk/poop) — Performance Optimizer Observation Platform This command line tool uses Linux's `perf_event_open` functionality to compare the performance of multiple commands with a colorful terminal user interface. It is similar to `hyperfine`.



## Multiple languages


- [CASR](https://crates.io/crates/casr) — Crash Analysis and Severity Report.

- [Code Pulse](http://code-pulse.com/) — Code Pulse is a free real-time code coverage tool for penetration testing activities by OWASP and Code Dx ([GitHub](https://github.com/codedx/codepulse)).

- [Daikon](https://plse.cs.washington.edu/daikon/) — A dynamic detector of likely invariants. An invariant is a property  that holds at a certain program point, often seen in assert statements,  documentation, and formal specifications.

- [Offensive360](https://offensive360.com) :copyright: — Offensive360 is an application security platform whose DAST engine crawls and tests running web applications and APIs (authenticated scanning, session handling, LLM-app testing) alongside SAST, SCA, mobile app scanning and malware/binary analysis. Findings map to OWASP Top 10 with remediation guidance. Deploys as SaaS or a self-hosted virtual appliance for offline/air-gapped networks. SARIF output; CI/CD and IDE integrations.

- [Sydr](https://sydr-fuzz.github.io/) :copyright: — Continuous Hybrid Fuzzing and Dynamic Analysis for Security Development Lifecycle.



<details>
<summary>Show Deprecated</summary>


- [allocscope](https://github.com/matt-kimball/allocscope) :warning: — allocscope is a tool for tracking down where the most egregiously large allocations are occurring in a C, C++ or Rust codebase. It is particularly intendend to be useful for developers who want to get a handle on excessive allocations and are working in a large codebase with multiple contributors with allocations occuring in many modules or libraries.



- [bytehound](https://github.com/koute/bytehound) :warning: — A memory profiler for Linux. Can be used to analyze memory leaks, see where exactly the memory is being consumed, identify temporary allocations and investigate excessive memory fragmentation.


</details>



## Other



<a id="api"></a>
<h2>API</h2>


- [Smartbear](https://smartbear.com/) :copyright: — Test automation and performance testing platform



<a id="binary"></a>
<h2>Binaries</h2>


- [angr](https://github.com/angr/angr) — Platform agnostic binary analysis framework from UCSB.

- [BAP](https://github.com/BinaryAnalysisPlatform/bap) — The Carnegie Mellon University Binary Analysis Platform (CMU BAP) is a suite of utilities and libraries for the analysis of machine code. The included Primus microexecution interpreter enables dynamic analysis.

- [Dr. Memory](https://drmemory.org/) — Dr. Memory is a memory monitoring tool capable of identifying memory-related programming errors ([Github](https://github.com/DynamoRIO/drmemory)).

- [DynamoRIO](http://www.dynamorio.org/) — Is a runtime code manipulation system that supports code transformations on any part of a program, while it executes.

- [llvm-propeller](https://github.com/google/llvm-propeller) — Profile guided hot/cold function splitting to improve cache efficiency. An alternative to BOLT by Facebook

- [Pin Tools](https://software.intel.com/en-us/articles/pin-a-dynamic-binary-instrumentation-tool) — A dynamic binary instrumentation tool and a platform for creating analysis tools.

- [TRITON](https://triton.quarkslab.com/) — Dynamic Binary Analysis for x86 binaries.



<details>
<summary>Show Deprecated</summary>


- [BOLT](https://github.com/facebookincubator/BOLT) :warning: — Binary Optimization and Layout Tool - A linux command-line utility used for optimizing performance of binaries  with profile guided permutation of linking to improve cache efficiency


</details>



<a id="bytecode"></a>
<h2>Bytecode/IR</h2>




<details>
<summary>Show Deprecated</summary>


- [souper](https://github.com/google/souper) :warning: — optimize LLVM IR with SMT solvers


</details>



<a id="cloud"></a>
<h2>Cloud</h2>


- [prowler](https://prowler.com/) — Prowler is an Open Source security tool to perform AWS and Azure security best practices assessments, audits, incident response, continuous monitoring, hardening and forensics readiness.
It contains hundreds of controls covering CIS, PCI-DSS, ISO27001, GDPR, HIPAA, FFIEC, SOC2, AWS FTR, ENS and custom security frameworks.



<a id="container"></a>
<h2>Containers</h2>


- [cadvisor](https://github.com/google/cadvisor) — Analyzes resource usage and performance characteristics of running containers.



<a id="laravel"></a>
<h2>Laravel</h2>




<details>
<summary>Show Deprecated</summary>


- [Enlightn](https://github.com/enlightn/enlightn) :warning: — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.


</details>



<a id="security"></a>
<h2>Security/DAST</h2>


- [AppScan Standard](https://www.hcltechsw.com/products/appscan) :copyright: — HCL's AppScan is a dynamic application security testing suite (previously by IBM)

- [WhiteHat Sentinel Dynamic](https://www.synopsys.com/software-integrity/security-testing/dast.html) :copyright: — Part of the WhiteHat Application Security Platform. Dynamic application security scanner that covers the OWASP Top 10.



<details>
<summary>Show Deprecated</summary>


- [Enlightn](https://github.com/enlightn/enlightn) :warning: — A static and dynamic analysis tool for Laravel applications that provides recommendations to improve the performance, security and code reliability of Laravel apps. Contains 120 automated checks.


</details>



<a id="web"></a>
<h2>Web</h2>


- [Smartbear](https://smartbear.com/) :copyright: — Test automation and performance testing platform



<a id="webassembly"></a>
<h2>WebAssembly</h2>


- [Wasabi](https://github.com/danleh/wasabi) — Wasabi is a framework for writing dynamic analyses for WebAssembly, written in JavaScript.



<a id="xml"></a>
<h2>XML</h2>


- [WhiteHat Sentinel Dynamic](https://www.synopsys.com/software-integrity/security-testing/dast.html) :copyright: — Part of the WhiteHat Application Security Platform. Dynamic application security scanner that covers the OWASP Top 10.



## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.

Title image [Designed by Freepik](https://www.freepik.com/).



