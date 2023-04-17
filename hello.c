#include <stdio.h>

struct person {
    char *name;
    int age;
};

int main() {
    int a = 2;
    int b = 3;
    int c = a + b;

    struct person p;
    p.name = "John";
    p.age = 20;

    if(c == 5){
        printf("equal");
    } else{
        printf("not equal");
    }
    
    printf("Hello, World!\n");
    return 0;
}
