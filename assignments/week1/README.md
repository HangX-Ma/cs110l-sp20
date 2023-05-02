# Week 1 Exercises

## Program 1: UPPERCASe

### Question 1

Did `clang-tidy` catch the problem? Based on what you know from lecture, why or why not? (There is no one correct answer we’re looking for here. Just speculate.)

- **Output**

    ```txt
    Error while trying to load a compilation database:
    Could not auto-detect compilation database for file "1-uppercase.c"
    No compilation database found in /home/mcontour/cs110l-win22/assignments/week1 or any parent directory
    fixed-compilation-database: Error while opening fixed database: No such file or directory
    json-compilation-database: Error while opening JSON database: No such file or directory
    Running without flags.
    ```

- **Answer**

    > No. As the `clang-tidy` documentation's introduction described, _Its purpose is to provide an extensible framework for diagnosing and fixing typical programming errors, like style violations, interface misuse, or bugs that can be deduced via static analysis._ The static analysis cannot find out the errors happened at runtime. Dataflow analysis is a typical way that a static analysis used.

### Question 2

Did Valgrind observe the issue? Based on what you know from lecture, why or why not?

- **output**

    ```txt
    ==9331== memcheck, a memory error detector
    ==9331== copyright (c) 2002-2017, and gnu gpl'd, by julian seward et al.
    ==9331== using valgrind-3.15.0 and libvex; rerun with -h for copyright info
    ==9331== command: ./1-uppercase hello\ world
    ==9331==
    hello world
    ==9331==
    ==9331== heap summary:
    ==9331==     in use at exit: 0 bytes in 0 blocks
    ==9331==   total heap usage: 1 allocs, 1 frees, 1,024 bytes allocated
    ==9331==
    ==9331== all heap blocks were freed -- no leaks are possible
    ==9331==
    ==9331== for lists of detected and suppressed errors, rerun with: -s
    ==9331== error summary: 0 errors from 0 contexts (suppressed: 0 from 0)
    ```

- **answer**

    > No. Valgrind is a dynamic analysis tools that can work with any binary compiled by any compiler. Valgrind’s `memcheck` stores some “shadow” metadata about heap memory accessed by your code — for example, bits indicating "this has been freed" or "this has been initialized". Using these, it can detect issues like double-freed heap blocks or uninitialized values. However, not a lot of information is available in binaries. For instance, it cannot detect stack-based buffer overflows.

### Question 3

Based on our discussions in lecture, why and how does AddressSanitizer find this error?

- **Output**

    ```txt
    =================================================================
    ==10275==ERROR: AddressSanitizer: dynamic-stack-buffer-overflow on address 0x7ffecc0f874b at pc 0x0000004c32dc bp 0x7ffecc0f8680 sp 0x7ffecc0f8678
    WRITE of size 1 at 0x7ffecc0f874b thread T0
        ...

    Address 0x7ffecc0f874b is located in stack of thread T0
    SUMMARY: AddressSanitizer: dynamic-stack-buffer-overflow /home/mcontour/cs110l-win22/assignments/week1/1-uppercase.c:9:17 in my_strcpy
    Shadow bytes around the buggy address:
    ...
    ```

- **Answer**

    > `AddressSanitizer` consists of a compiler instrumentation module and a run-time library. It has the same idea as the valgrind but it process the source code so more information is available at pre-compilation. Therefore, it can finds use of improper memory addresses: out of bounds memory accesses, double free, use after free, etc. `1-uppercase.c` contains the _out of bounds memory accesses_ problem.

### Question 4

Based on our discussions in class, speculate about why clang-tidy might report this false positive for me.

- **Answer**

  > The data flow checking style just like _if-else_ statements. It will follow each branch, even if it’s impossible for some condition to be true in real life. Therefore, `clang-tidy` can find the condition error but cannot find the memory leak.

### Question 5

What do you find?

- **Output**

    ```txt
    =================================================================
    ==12085==ERROR: LeakSanitizer: detected memory leaks

    Direct leak of 32 byte(s) in 2 object(s) allocated from:
        ...
    SUMMARY: AddressSanitizer: 32 byte(s) leaked in 2 allocation(s).
    ```

- **Answer**

    > Memory leak is found by AddressSanitizer and it print out the backtrace information.

### Question 6

What do you find?

- **Output**

    ```txt
    warning: Potential leak of memory pointed to by 'mutable_copy'
    ```

- **Answer**

    > `clang-tidy` finds out the forgotten memory freeing function.

### Question 7

- Do the sanitizers catch any problems? Why or why not?
    > Actually Not. Sanitizers only follow the code path of the right input.
- Can you make the sanitizers catch any problems? (Hint: try running the program on a different input!)
  - **Input**

    ```bash
    ./3-bracket-parser 'hi [hello world!'
    ```

  - **Output**

    ```txt
    Malformed input!

    =================================================================
    ==16024==ERROR: LeakSanitizer: detected memory leaks

    Direct leak of 17 byte(s) in 1 object(s) allocated from:
        ...
    SUMMARY: AddressSanitizer: 17 byte(s) leaked in 1 allocation(s).
    ```

### Question 8

What input did your fuzzer generate?

- **Answer**

    ```txt
    [[^@^@
    ```

### Question 9

None of the tools – `clang-tidy`, `valgrind`, or the sanitizers – find any problems with this file. Why don’t they detect this problem?

- **Answer**
    > This is a logic problem. These tools only analysis the problems that cause corruption or program vulnerability.
