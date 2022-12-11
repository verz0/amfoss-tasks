#include <stdio.h>
#include <math.h>
#include <cs50.h>

int main()
{
    float dollars;
    do
    {
        dollars = get_float("change owed : ");

    }
    while (dollars < 0);
    int cents = round(dollars*100);
    int coins = 0;
    while (cents !=0)
    {
        coins += cents/25;
        cents = cents%25;

        coins += cents/10;
        cents %= 10;

        coins += cents/5;
        cents %= 5;

        coins += cents/1;
        cents %=1;
    }
    printf("%d\n", coins);

}
