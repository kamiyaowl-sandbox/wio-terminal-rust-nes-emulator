typedef struct HelloStruct {
    unsigned int a;
    unsigned int b;
    unsigned int c;
    unsigned int d;
} HelloStruct;

unsigned int hello(const HelloStruct* src) {
    return 0xaa995566 & (src->a | src->b | src->c | src->d);
}
