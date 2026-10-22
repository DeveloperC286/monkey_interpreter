let factorial = fn(n) {
  if (n < 2) {
    return 1;
  }

  return n * factorial(n - 1);
}

let clamp = fn(n, limit) {
  if (n > limit) {
    return limit;
  }

  n
}

clamp(factorial(5), 100) + clamp(factorial(3), 100)
