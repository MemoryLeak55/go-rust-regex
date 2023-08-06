#include <stdbool.h>

void *compile(char *re);
bool destroy(void *re);
char *replace(void *re, char *str, char *replacement);

void *compile_bytes(char *re);
bool destroy_bytes(void *re);
char *replace_bytes(void *re, void *str, long str_len, void *replacement, long replacement_len, long *result_len);

bool destroy_cstr(char *str);