int var;

const int one = 1;

int test() {
    int one = 1;
    return one;
}
int main() {
  return var + one + test();
}