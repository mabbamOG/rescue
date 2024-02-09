mod field;
use field::Fp64 as Fp;

fn rescue_hash_raw(input: [Fp; 8]) -> [Fp; 4] {
    let mut state: [Fp; 12] = [Fp::from(0); 12];
    for i in 0..8 {
        state[i] = input[i];
    }
    let subkeys: [[Fp;12]; 21] = todo!();
    state = vector_add(state, subkeys[0]);
    for r in 0..10 {
        state = rescue_round(state, (subkeys[2*r + 1], subkeys[2*r + 2]));
    }
    [state[0], state[1], state[2], state[3]]
}

fn vector_add(mut lhs: [Fp;12], rhs: [Fp;12]) -> [Fp; 12] {
    for i in 0..12 {
        lhs[i] = lhs[i] + rhs[i];
    }
    lhs
}

fn rescue_round(mut state: [Fp;12], (subkey1, subkey2): ([Fp;12], [Fp;12])) -> [Fp;12] {
    const MDS: [[Fp;12]; 12] = todo!();
    state = vector_cbrt(state);
    state = matrix_mul(MDS, state);
    state = vector_add(state, subkey1);
    state = vector_cube(state);
    state = matrix_mul(MDS, state);
    state = vector_add(state, subkey2);
    state
}

fn field_cbrt(x: Fp) -> Fp {
    const TWO_P_MINUS_ONE: u64 = (Fp::PRIME_MODULUS*2 - 1)/3;
    x.pow(TWO_P_MINUS_ONE)
}

fn vector_cube(mut v: [Fp;12]) -> [Fp;12] {
    for i in 0..12 {
        v[i] = v[i].pow(3);
    }
    v
}

fn vector_cbrt(mut v: [Fp;12]) -> [Fp;12] {
    for i in 0..12 {
        v[i] = field_cbrt(v[i]);
    }
    v
}

fn matrix_mul(M: [[Fp;12]; 12], v: [Fp;12]) -> [Fp;12] {
    fn vector_mul(lhs: [Fp;12], rhs: [Fp;12]) -> Fp {
        let mut output = Fp::from(0);
        for i in 0..12 {
            output = output + lhs[i]*rhs[i];
        }
        output
    }

    let mut output = [Fp::from(0); 12];
    for i in 0..12 {
        output[i] = vector_mul(M[i], v);
    }
    output
}



fn main() {
    println!("Hello, world!");
}
