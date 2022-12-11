#include <stdio.h>
#include <cs50.h>

int main()
{
    int height;
    do
    {
        height=get_int("enter the width noob: ");

    }
    while (!(height>=1 && height<=8));
    for (int row =1; row <= height; row++)
    {
        for (int k=1; k<= height-row;k++){
            printf(" ");
        }
        for (int j=1; j <= row; j++)
        {
            printf("#");
        }
        printf("\n");
    }
    return  0;
}
