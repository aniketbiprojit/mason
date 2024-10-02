#include <stdio.h>

const x = 2;

struct person {
    char *name;
    int age;
};

int arr[4] = {1,2,3,4};

int main() {
    int a = 2;
    int b = 3;
    int c = a + b;

    float f = 8.6;

    struct person p;
    p.name = "John";
    p.age = 20;

    if(c == 5){
        printf("equal");
    } else{
        printf("not equal to 5 != %d", c);
    }
    
    printf("Hello, World!\n");
    a==b;
    return 0;
}
