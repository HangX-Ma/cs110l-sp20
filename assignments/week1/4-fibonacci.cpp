#include <iostream>
#include <string>

int fib(int n) {
    int t1 = 0;
    int t2 = 1;

    for (int i = 1; i <= n; i++) {
        int next = t1 + t2;
        t1 = t2;
        t2 = next;
    }

    return t2;
}

int main() {
    while (true) {
        std::cout << "Enter a number (or ctrl+c to exit): " << std::flush;
        std::string input;
        getline(std::cin, input);

        int parsed;
        try {
            parsed = std::stoi(input);
        } catch (std::invalid_argument &e) {
            std::cout << "That's not a valid number." << std::endl;
            continue;
        }

        std::cout << "fib(" << parsed << ") = " << fib(parsed) << std::endl;
    }
}
