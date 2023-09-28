use num_bigint;
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand;


#[derive(Debug, Clone)]
pub struct ShamirSecretShare {
    pub n: usize, // threshold shares required to open the secret
    pub k: usize, // total number of shares generated from the polynomial
    pub p: BigInt // polynomial to work in the prime field GF(P)
}

impl ShamirSecretShare {
    pub fn split(&self, secret: BigInt) -> Vec<(usize, BigInt)> {
        assert!(self.n < self.k, "n should be less than total shares k");
        let polynomial = self.sample_polynomial(secret);
        self.evaluate_polynomial(polynomial)
    }


    // returns a vector with a random larger degree coefficients with secret as constant term 
    fn sample_polynomial(&self, secret: BigInt) -> Vec<BigInt> {
        let mut coeff = vec![secret];
        let mut rng = rand::thread_rng();
        let l = BigInt::from(0);
        let h = &self.p - BigInt::from(1);
        let rand_coeff: Vec<BigInt> = (0..(self.n - 1)).map(|_| rng.gen_bigint_range(&l, &h)).collect();
        coeff.extend(rand_coeff);
        coeff
    }


    fn evaluate_polynomial(&self, polynomial: Vec<BigInt>) -> Vec<(usize, BigInt)> {
        (1..=self.k)
            .map(|x| (x, self.mod_evaluate_at(&polynomial, x)))
            .collect()
    }

    fn mod_evaluate_at(&self, polynomial: &[BigInt], x: usize) -> BigInt {
        let x_bigint = BigInt::from(x);
        polynomial.iter().rev().fold(Zero::zero(), |sum, item| {
            (&x_bigint * sum + item) % &self.p
        })
    }

    pub fn recover(&self, shares: &[(usize, BigInt)]) -> BigInt {
        assert!(shares.len() == self.n, "wrong shares number");
        let (xs, ys): (Vec<usize>, Vec<BigInt>) = shares.iter().cloned().unzip();
        let result = self.lagrange_interpolation(Zero::zero(), xs, ys);
        if result < Zero::zero() {
            result + &self.p
        } else {
            result
        }
    }

    fn lagrange_interpolation(&self, x: BigInt, xs: Vec<usize>, ys: Vec<BigInt>) -> BigInt {

        let n = xs.len();
      
        let mut result = Zero::zero();
      
        for i in 0..n {
          let mut term = ys[i].clone();
      
          for j in 0..n {
            if i != j {
              term = term * (x.clone() - BigInt::from(xs[j])) % &self.p;
              
              let xi = BigInt::from(xs[i]);
              let xj = BigInt::from(xs[j]);
              let temp = xi - xj;
              
              term = term * self.modular_inverse(temp) % &self.p;
            }
          }
      
          result = (result + term) % &self.p;
        }
      
        result
      
      }
      

      // uses stein algorithm to find modular inverse which has good performance in modern hardware
      pub fn modular_inverse(&self, number: BigInt) -> BigInt {
        
        let mut a = number;
        let mut m = self.p.clone();
        
        let mut x: BigInt = Zero::zero();
        let mut x1 = One::one();
      
        while m > One::one() {
          
          let q = a.clone() / m.clone();    
          let temp_m = m.clone();
          
          m = a % m.clone();
          a = temp_m;
      
          let temp_x = x.clone();
          x = x1 - q * x.clone();
          x1 = temp_x;
        }
        
        while x < Zero::zero() {
          x = x + &self.p;
        }
      
        x
      }


}
