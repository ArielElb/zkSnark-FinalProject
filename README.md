
## Main idea:

- In cryptography there are many settings in which we use prime numbers.
- Those prime numbers need to be agreed upon.
- We don’t want the client to compute the prime number.
- We don’t want the server to have full control on the choice of the prime number.

## The Protocol:

- The client samples a random number 𝑥∈ℱ .
- The server knows 𝑥 and the client and server agree on some hash function ℎ.
- Now the server wants to do the complex computation and check if  ℎ(𝑥+𝑎)  is a prime number using Miller-Rabin algorithm.
- We want to return the smallest 𝑎 such that ℎ(𝑥+𝑎) is prime number.
- Ideally, we want 𝑎=0 such that ℎ(𝑥) is prime number But ℎ(𝑥)  is not necessarily a prime number.
So, we will calculate h(x),ℎ(𝑥+1),ℎ(𝑥+2),…..,ℎ(𝑥+𝑎−1),ℎ(𝑥+𝑎).

- The server wants to provide a succinct proof to the client such that:
   -h(x),ℎ(𝑥+1),ℎ(𝑥+2),…..,ℎ(𝑥+𝑎−1) are not prime numbers
   -ℎ(𝑥+𝑎) is prime number.

-The client wants to verify this proof in a short time and will be convinced that:
  -h(x),ℎ(𝑥+1),ℎ(𝑥+2),…..,ℎ(𝑥+𝑎−1) are not prime numbers
  -ℎ(𝑥+𝑎) is prime number.
- By the Prime Number theorem  we get that It wont take long for the prover to find a prime number.

## Tech:

arkworks is a Rust ecosystem for zkSNARK programming. 
Libraries in the arkworks ecosystem provide efficient implementations of all components required to implement zkSNARK applications, from generic finite fields to R1CS constraints for common functionalities.
R1CS is a NP-complete language that will help us represent our calculation.



