/* 
 * # Task :
 *   write a program that will print a histogram of lengths of words from input
 */

#include <stdio.h>


// Maximum amount of words that can be handled by the program
#define MAX_WORD_LENGTH 21
#define HISTOGRAM_HEIGHT 10


int main() {
    char ch = 0; // -128 <-> 127
    unsigned 
        last_word_len = 0,
        // Contains count of times each word with a specific length is met
        // last item will contain all other words
        // which length exceeds the MAX_WORD_LENGTH
        word_lengths[MAX_WORD_LENGTH] = {0},
        most_frequent_word_count = 0;


    printf("-------------------------------------------------------------\n"
           "Take into account that only words with length up to %d will "
           "be counted.\n"
           "Other words which length is larger than MAXIMUN mentioned above "
           "will be shown in the last column of histogram\n"
           "Please type your input and then press Ctrl+D to get the results:\n"
           "-------------------------------------------------------------\n",
           
           MAX_WORD_LENGTH - 1
    );

    // Count words with specific lenghts and save counts to the array
    //
    while ((ch = getchar()) != EOF) {
        if ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')) {
            ++last_word_len;
            continue;
        } else if (last_word_len > 0) {
            if (last_word_len > MAX_WORD_LENGTH) 
                last_word_len = MAX_WORD_LENGTH;
        
            word_lengths[last_word_len - 1] += 1;

            // Detect most frequent word and store amount of times it was met
            // we'll need this to evenly distribute frequency on our histogram
            if (most_frequent_word_count < word_lengths[last_word_len - 1]) {
                most_frequent_word_count += 1;
            }
        }

        last_word_len = 0;
    }

    // Ratio between the maximum height of histogram
    // to the most frequent word count
    // if the <wc> (word count) is larger 
    float histoh_to_wc_ratio = 1;
    if (most_frequent_word_count > HISTOGRAM_HEIGHT) {
        histoh_to_wc_ratio = (float) HISTOGRAM_HEIGHT / most_frequent_word_count;
    }

    printf("\n| K: %f | Most freq: %u\n\n" ,
           
           histoh_to_wc_ratio,
           most_frequent_word_count
    );

    for (unsigned i = HISTOGRAM_HEIGHT; i > 0; --i) {
        // convert current histo row number to word count
        // according to ratio
        unsigned wlen = (float) (i / histoh_to_wc_ratio);

        printf("| %12u | %12s |", wlen, "->");
        for (unsigned j = 0; j < MAX_WORD_LENGTH; ++j) {
            if (word_lengths[j] >= wlen) {
                printf(" %3c |", '*');
            } else {
                printf(" %3c |", ' ');
            }
        }
        printf("\n");
    }

    for (unsigned i = 0; i < 32 + MAX_WORD_LENGTH * 6; ++i)
        printf("%c", '-');
    printf("\n| Word Count ^ | Word length: |");
    for (unsigned i = 0; i < MAX_WORD_LENGTH; ++i) {
        printf(" %3d |", i);
    }
    printf("\n\n");

}