# Note: this Makefile was adapted from the CS 110 assignment 3 Makefile

C_PROGS = 1-uppercase 2-linkedlist 3-bracket-parser
CXX_PROGS = 4-fibonacci
PROGS = $(C_PROGS) $(CXX_PROGS)

# Choosing a compiler: We want LLVM (clang) so that we can use sanitizers
# NOTE: if you are not running on myth, you may need to change this.
CC = clang-10
CXX = clang++-10

# Flags for C compiler:
CFLAGS = -g -O0 -Wall -Wextra -std=gnu99 -fsanitize=address,leak,undefined
# Explanation of flags:
# * -g: keep debug symbols in the binary (makes it easier to run in gdb)
# * -O0: optimization level 0 (turn off most optimizations)
# * -Wall: Turn on "all" warnings (this name is a complete lie)
# * -Wextra: Turn on even more warnings (because -Wall doesn't include them all)
# * -std=c11: Use the 1999 C standard with GNU extensions

# Flags for C++ compiler:
CXXFLAGS = -g -O0 -Wall -Wextra -std=c++17 -fsanitize=address,leak,undefined
# Explanation of flags:
# * -g: keep debug symbols in the binary (makes it easier to run in gdb)
# * -O0: optimization level 0 (turn off most optimizations)
# * -Wall: Turn on "all" warnings (this name is a complete lie)
# * -Wextra: Turn on even more warnings (because -Wall doesn't include them all)
# * -std=c++17: Use the 2017 C++ standard

# Flags for the linker. The compiler translates each .c or .cpp file into a
# compiled "object" file (.o), and the linker takes all the .o files and
# combines them into an executable.
LDFLAGS = -fsanitize=address,leak,undefined

# This fancy Makefile code takes strings like "1-uppercase" and generates file
# paths like "1-uppercase.c" and "1-uppercase.o"
C_PROGS_SRC = $(patsubst %,%.c,$(C_PROGS))
C_PROGS_OBJ = $(patsubst %.c,%.o,$(patsubst %.S,%.o,$(C_PROGS_SRC)))
CXX_PROGS_SRC = $(patsubst %,%.cpp,$(CXX_PROGS))
CXX_PROGS_OBJ = $(patsubst %.cpp,%.o,$(patsubst %.S,%.o,$(CXX_PROGS_SRC)))

default: $(PROGS)

$(C_PROGS): %:%.o
	$(CC) $^ $(LDFLAGS) -o $@

$(CXX_PROGS) $(EXTRA_CXX_PROGS): %:%.o
	$(CXX) $^ $(LDFLAGS) -o $@

clean::
	rm -fr $(C_PROGS) $(C_PROGS_OBJ)
	rm -fr $(CXX_PROGS) $(CXX_PROGS_OBJ)

.PHONY: all clean
