#include <stdio.h>
#include <uv.h>

int main() {
    for (int i = 0; i > -10; i--) {
        printf("%s: %s\n", uv_err_name(i), uv_strerror(i));
    }
}
