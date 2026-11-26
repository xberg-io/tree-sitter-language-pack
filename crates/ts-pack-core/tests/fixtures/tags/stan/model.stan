functions {
  real square(real x) {
    return x * x;
  }
}
data {
  int<lower=0> N;
  vector[N] y;
}
parameters {
  real mu;
}
model {
  y ~ normal(mu, 1);
}
