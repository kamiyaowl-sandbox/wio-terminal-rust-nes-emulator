typedef struct HelloStruct {
    int a;
    int b;
    int c;
    int d;
} HelloStruct;

int hello(const HelloStruct* src) {
    return 0xaa995566 & (src->a | src->b | src->c | src->d);
}
