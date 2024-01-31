// Counts spaces, tabs and end-line symbols in the input

#include <stdio.h>


int main ()
{
    int c;
    double ns = 0, // spaces counter
           nt = 0, // tabs counter
           nn = 0; // end-line counter
    
    while ((c = getchar()) != EOF)
        if (c == ' ') ++ns;
        else if (c == '\t') ++nt;
        else if (c == '\n') ++nn;

    printf("Spaces: (%.0f) | Tabs: (%.0f) | Newlines: (%.0f)", ns, nt, nn);

    return 0;
}