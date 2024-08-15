#include <stdint.h>


inline uint32_t ip_bin(uint64_t *x, uint64_t *y, uint32_t len) {
    uint64_t ret = 0;
    for (uint32_t i = 0; i < len; i++) {
        ret += __builtin_popcountll(x[i] & y[i]);
    }
    return ret;
}

inline uint32_t ip_bin_16(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (uint32_t i = 0; i < 15; i++) {
        ret += __builtin_popcountll(x[i] & y[i]);
    }
    return ret;
}

inline uint32_t ip_bin_120(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (uint32_t i = 0; i < 120; i++) {
        ret += __builtin_popcountll(x[i] & y[i]);
    }
    return ret;
}

inline uint32_t ip_bin_200(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (uint32_t i = 0; i < 200; i++) {
        ret += __builtin_popcountll(x[i] & y[i]);
    }
    return ret;
}

uint32_t ip_byte_bin(uint64_t *x, uint64_t *y, uint32_t len) {
    uint64_t ret = 0;
    for (int i = 0; i < 4; i++) {
        ret += ip_bin(x, y, len) << i;
        y += 16;
    }
    return ret;   
}

uint32_t ip_byte_bin_16(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (int i = 0; i < 4; i++) {
        ret += ip_bin_16(x, y) << i;
        y += 16;
    }
    return ret;   
}

uint32_t ip_byte_bin_120(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (int i = 0; i < 4; i++) {
        ret += ip_bin_120(x, y) << i;
        y += 16;
    }
    return ret;   
}

uint32_t ip_byte_bin_200(uint64_t *x, uint64_t *y) {
    uint64_t ret = 0;
    for (int i = 0; i < 4; i++) {
        ret += ip_bin_200(x, y) << i;
        y += 16;
    }
    return ret;   
}