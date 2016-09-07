#define CHUNK 1024
#include <stdio.h>

int main(int argc, char** argv) {
    char buf[CHUNK];
    size_t nread;
    FILE *file;
    
    file = fopen(argv[1], "r");
    if (file) {
        while ((nread = fread(buf, 1, sizeof buf, file)) > 0) {
            fwrite(buf, 1, nread, stdout);
        }

        if (ferror(file)) {
            printf("error.\n");
        }

        fclose(file);
    }
}
