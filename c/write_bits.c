#include "stdlib.h"
#include "stdio.h"

void write_bits(char* bits, char* to) {
    // no chk bit be '0' or '1'
    int i = 0, len = (int)strlen(bits);
    int cur_val = 0, cur_pos = 0;
    for (; i < len; i++) {
        // input simulate little-end
        cur_val = ((bits[i] - '0') << (i%8)) + cur_val;
        // input simulate big-end
        // cur_val = (cur_val << 1) + (bits[i] - '0');
        if ((i+1) % 8 == 0) {
            to[cur_pos++] = cur_val;
            cur_val = 0;
        }
    }
    if (cur_val) to[cur_pos] = cur_val;
    return;
}

int main() {
    while (1) {
        printf("Enter bits: ");
        char bits[128];
        // unsafe
        scanf("%s", bits);
        char *int_ptr = malloc(sizeof(int));
        write_bits(bits, int_ptr);
        printf("%d\n", *((int*)int_ptr));
    }
}
