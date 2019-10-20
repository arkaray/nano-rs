#include <stdio.h>
#include <stdbool.h>

void utf8_init(void);
bool using_utf8(void);
bool is_byte(int c);
bool is_ascii_cntrl_char(int c);

int main(int argc, char **argv) {
    printf("utf8:%s\n",using_utf8()?"yes":"no");
    utf8_init();
    printf("utf8:%s\n",using_utf8()?"yes":"no");
    printf("8 is %s while 1000 is %s\n",
                                        is_byte(8)?"a byte":"not a byte",
                                        is_byte(1000)?"a byte":"not a byte");
    printf("newline is a %s while c is a %s\n",
                                            is_ascii_cntrl_char('\n')?"Control":"Character",
                                            is_ascii_cntrl_char('c')?"Control":"Character");
    return 0;
}