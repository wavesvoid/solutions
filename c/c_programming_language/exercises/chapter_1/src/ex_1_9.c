// Replace more than one character in input string with a single one

#include <stdio.h>


int main ()
{
    int c;
    char last = 'a'; // some random initialisation symbol other than space

    for (; (c = getchar()) != EOF; last = c)
        if (c != ' ' || last != ' ')
            putchar(c);
    
    return 0;
}