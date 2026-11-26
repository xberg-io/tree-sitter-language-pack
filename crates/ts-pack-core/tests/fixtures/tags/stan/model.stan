functions {
  real square(real x) {
    return x * x;
  }

  real scaled_normal_lpdf(vector y, real mu, real sigma) {
    return normal_lpdf(y | mu, square(sigma));
  }
}
data {
  int<lower=0> N;
  vector[N] y;
}
transformed data {
  real y_mean = mean(y);
}
parameters {
  real mu;
  real<lower=0> sigma;
}
transformed parameters {
  real mu_centered = mu - y_mean;
}
model {
  real local_scale = 2.0;
  mu ~ normal(0, local_scale);
  y ~ scaled_normal(mu, sigma);
  if (sigma > 100) {
    reject("sigma too large: ", sigma);
  }
}
generated quantities {
  real y_rep = normal_rng(mu, sigma);
  print("mu = ", mu);
}
