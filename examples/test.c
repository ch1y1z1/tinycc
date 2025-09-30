int add(int x, int y) {
  int ans = x + y;
  return ans;
}

int sign(int x) {
  if (x > 0) {
    return 1;
  } else if (x < 0) {
    return -1;
  } else {
    return 0;
  }
}

int main() {
  int a = 3;
  int b;
  b = add(a, 7);
  int c = sign(b - 10);
  return c;
}
