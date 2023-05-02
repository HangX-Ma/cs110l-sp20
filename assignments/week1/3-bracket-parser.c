#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

void parse(const char* input) {
    // Goal: parse out a string between brackets
    // (e.g. "   [target string]" -> "target string")

    char *mutable_copy = strdup(input);

    // Find open bracket
    char *open_bracket = strchr(mutable_copy, '[');
    if (open_bracket == NULL) {
        printf("Malformed input!\n");
        free(mutable_copy);
        return;
    }

    // Make the output string start after the open bracket
    char *parsed = open_bracket + 1;

    // Make sure there is at least one character in brackets
    if (parsed[0] == '\0') {
        printf("There should be at least one character in brackets!\n");
        free(mutable_copy);
        return;
    }

    // Find the close bracket
    char *close_bracket = strchr(parsed, ']');
    if (close_bracket == NULL) {
        printf("Malformed input!\n");
        return;
    }

    // Replace the close bracket with a null terminator to end the parsed
    // string there
    *close_bracket = '\0';

    printf("Parsed string: %s\n", parsed);
    free(mutable_copy);
}

#if 0
int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <string to parse>\n", argv[0]);
        return 1;
    }

    parse(argv[1]);

    return 0;
}
#endif

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    // TODO (instructions below): do something with `data` and `size`
    char buf[size + 1];
    memcpy(buf, data, size);
    buf[size] = '\0';
    parse(buf);
    // libFuzzer functions always return 0
    return 0;
}

