#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

void my_strcpy(char *dest, const char *src) {
    size_t i = 0;
    while (true) {
        dest[i] = src[i];
        if (src[i] == '\0') {
            break;
        }
        i++;
    }
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <string to uppercase>\n", argv[0]);
        return 1;
    }
    
    char uppercase[strlen(argv[1])];    // no space for null terminator :(

    // I reimplemented strcpy because I want to see if the static analyzer can
    // find this bug without relying on "don't call strcpy" as a crutch
    my_strcpy(uppercase, argv[1]);

    // Make the string uppercase
    for (int i = 0; uppercase[i] != '\0'; i++) {
        if(uppercase[i] >= 'a' && uppercase[i] <= 'z') {
            uppercase[i] = uppercase[i] - ('a' - 'A');
        }
    }

    printf("%s\n", uppercase);

    // Remember, a one-byte buffer overflow is enough to completely compromise
    // a program!
}
