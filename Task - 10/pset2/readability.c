#include <stdio.h>
#include <cs50.h>
#include <string.h>
#include <ctype.h>
#include <math.h>

int get_index(string s);

int main() {
    string text = get_string("enter text noob: ");
    int index = get_index(text);

    if (index<1){
        printf("before grade 1\n");

    } else if (index >=16) {
        printf("grade 16+\n");

    } else {
        printf("grade %d\n", index);
    }
    return 0;
}

int get_index(string s){
    int letters=0,sentences=0,words=0;

    for(int i =0; i<strlen(s); i++){
        char ch = s[i];
        if (isalpha(ch)) {
            letters++;
        }

        if (isspace(ch)){
            words++;
        }

        if (ch == '.' || ch =='!' || ch == '?' ){
            sentences++;
        }
    }
    words++;

    float L=(letters * 100.0f)/ words;
    float S=(sentences * 100.0f)/ words;
    return round(0.0588*L-0.296*S-15.8);

}
