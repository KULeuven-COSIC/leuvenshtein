use std::mem;

use tfhe::shortint::gen_keys;
use tfhe::shortint::parameters::*;

use std::time::Instant;

/*
    Code used for the results of the Levenshtein distance paper
*/

fn gen_vec() -> Vec<u64> {
    let mut vec: [u64; 16] = [0u64; 16];

    for vin in 0..3 {
        for hin in 0..3 {
            for eq in 0..2 {
                let key = vin + 3 * hin + 9 * eq;

                if key < 16 {
                    vec[key] = (1 + std::cmp::min(
                        std::cmp::min(-(eq as i32), (vin as i32) - 1),
                        (hin as i32) - 1,
                    )) as u64;
                }
            }
        }
    }
    vec.to_vec()
}

fn gen_vec_eq() -> Vec<u64> {
    let mut vec: [u64; 16] = [0u64; 16];
    vec[0] = 9;

    vec.to_vec()
}

fn gen_vec_1eq() -> Vec<u64> {
    let mut vec: [u64; 16] = [0u64; 16];
    vec[0] = 1;

    vec.to_vec()
}

fn levenshtein_plain(x: &str, y: &str) -> Vec<u32> {
  let xlen = x.len();
  let ylen = y.len();

  let str1 = x.bytes().collect::<Vec<u8>>();
  let str2 = y.bytes().collect::<Vec<u8>>();

  let vec_size = std::cmp::max(xlen + 1 as usize, ylen + 1 as usize);
  let mut current: Vec<u32> = Vec::with_capacity(vec_size);
  let mut prev: Vec<u32> = Vec::with_capacity(vec_size);

  for i in 0..vec_size {
      current.push(0u32);
      prev.push(i as u32);
  }

  for j in 0..ylen {
      current[0] = (j + 1) as u32;

      for i in 0..xlen {
          let ins = current[i] + 1;
          let dlt = prev[i + 1] + 1;
          let mut sub = prev[i];
          if str1[i] != str2[j] {
              sub += 1;
          }

          current[i + 1] = std::cmp::min(std::cmp::min(dlt, ins), sub);
      }
      mem::swap(&mut current, &mut prev);
  }
  prev

  // anwser sits in previous[vec_size]
}


fn main() {

    // length 8
    let x = "abcdefgh";
    let y = "adcdefgf";

    // length 100
    // let x = "BEgfEHGfShHtvKazXNeEvNWmvfbrAWyAYZjkXvNkmEajQNCTKZnkPeEDadvQtUnGhJRpWZASUMfXArGZFSUYgFeCAWxSvKNdpsnV";
    // let y = "EEhjZMnFsMFCsKnnyZvtrPeKxfmJfJVJNcYAwYmrGNgTUUSAgduNQZttWFdYFKddcKjkUEpPUmGkszZSVVNWkThxSFRgMzrbqATe";

    // length 210
    // let x = "xEd4he4aWLy70e8FDS65tABZvBq9ZMu25td9mKXdWGBtQa5TV7PE7EDtQkDbqXYngLQEBBnkGg5yiMPGNz48M139cqj1GCZx97YbFXA9PrYKYUuMdi8mSeUgb5TAgc800zDGmrfeMygzMS8LmRzcaaPdRL0Zw9NGDMk5EV1VY1jZ18H848buQtNjGtcenHneAnC3YUb0rm3JKSDMmg";
    // let y = "SD67VLrxVyt4Tag4CYQPQfqxWXn5zz1SaNkGHH0diEnC7nQyemQktYiUgXT0Nx7mp9gzfEvCYXNUR8MbW8JYeCgCJYmLEmiqhBGFQYZTenm6Jb5YhBrq4kb2iQk6yXDA23Gf32HFn3fmZF6Tv73UZF6WHgdLhB3GnzzvY6DNJ7UG5bUUzDDc8L1pUiMS55qfH1mLZQPDQYwQDixX5E";

    // lenght 256
    // let x = "mizf30WE1Pzqu0HtZKMMCE6f3NE2TGPPYmgSunfkBJqGJqveg97i61fu5KG7z8UmR5DVVALk5CCL0fzEv687LdRuunZ8SYUFmQEf66dXZ6vejGKR7HhSiY5XLWbCYFddqtFEX2QjmHRmqB7tngfG7m0CBX1D6wcvLyYp0rpiv1GSw5T1ZfuT2mcjHUi05zd6N4EqLmTP8ETEc2vQaJcVGSaXRETejNebwwb4m9wUUMd0abrEQeLw3Ubn9un6tTeP";
    // let y = "yVL0hi1V8mUQakFWZGEHwQBduvaTtKC6dNMbnyEiU29pKdwLfCnY7jWceXQizTiwhGxiAkuwvdcgpTtaZW3XJ6t75HL86nV2QPUWaxipf6x7JE8NPQT0RrzcheaydjLdPyUhAQ3UhJ2bLVE5wtNDAgdBgX3N5Ru4iXqbFXWD5ZAbzniVaWr5iE0wQenwt8QjqERf6A67P9rLkmGPKS8LHJxttCKRBWM3qr1F93JZtEhGKcQ1079pUXcCgAvLhCWi";

    let m = x.len();
    let n = y.len();

    println!("First string[{}]: {}", m, x);
    println!("Second string[{}]: {}", n, y);

    let lut_vec = gen_vec(); // [0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0]
    let lut_vec_eq = gen_vec_eq();
    let lut_vec_1eq = gen_vec_1eq();

    let (cks, sks) = gen_keys(PARAM_MESSAGE_4_CARRY_0_KS_PBS);

    // Encryption of the input strings
    let scale_factor: u8 = 0; // You can put it to 64

    let mut x_enc = x
        .bytes() // convert char to int
        .map(|c| cks.encrypt((c - scale_factor) as u64)) // Encrypts
        .collect::<Vec<tfhe::shortint::Ciphertext>>();

    let mut y_enc = y
        .bytes() // convert char to int
        .map(|c| cks.encrypt((c - scale_factor) as u64)) // Encrypts
        .collect::<Vec<tfhe::shortint::Ciphertext>>();

    let mut x2_enc = x
        .bytes() // convert char to int
        .map(|c| cks.encrypt(((c - scale_factor) >> 4) as u64)) // Encrypts
        .collect::<Vec<tfhe::shortint::Ciphertext>>();

    let mut y2_enc = y
        .bytes() // convert char to int
        .map(|c| cks.encrypt(((c - scale_factor) >> 4) as u64)) // Encrypts
        .collect::<Vec<tfhe::shortint::Ciphertext>>();

    // Build up of the matrices
    let xlen = m + 1;
    let ylen = n + 1;

    let mut h_matrix: Vec<Vec<tfhe::shortint::Ciphertext>> = Vec::with_capacity(xlen);
    let mut v_matrix: Vec<Vec<tfhe::shortint::Ciphertext>> = Vec::with_capacity(xlen);

    let zero_enc = cks.encrypt(0u64);
    let one_enc = cks.encrypt(1u64);

    for _ in 0..xlen {
        let mut vec: Vec<tfhe::shortint::Ciphertext> = Vec::with_capacity(ylen);
        for _ in 0..ylen {
            vec.push(zero_enc.clone());
        }
        h_matrix.push(vec.clone());
        v_matrix.push(vec.clone());
    }

    for i in 0..xlen {
        v_matrix[i][0] = cks.encrypt(1u64);
    }
    for i in 0..ylen {
        h_matrix[0][i] = cks.encrypt(1u64);
    }

    // Determine threshold
    let th = m/2.0 as usize;     
    // let th = f64::ceil(m as f64 * 0.25) as usize;

    // Disable the threshold
    // let th = m+2; 

    let start = Instant::now();

    for i in 1..xlen {
      let t = Instant::now();
        for j in 1..ylen {
            if usize::abs_diff(i ,j) <= th {

                let lut_1eq = sks.generate_lookup_table(|x| lut_vec_1eq[x as usize]);
                let lut_eq = sks.generate_lookup_table(|x| lut_vec_eq[x as usize]);

                // Check the first part of the character
                let mut eq1 = sks.unchecked_sub(&mut x_enc[i - 1], &mut y_enc[j - 1]);
                sks.unchecked_scalar_add_assign(&mut eq1, 16);

                sks.apply_lookup_table_assign(&mut eq1, &lut_1eq);
                eq1 = sks.unchecked_sub(&one_enc, &eq1);
                sks.unchecked_scalar_add_assign(&mut eq1, 16);

                let mut eq2 = sks.unchecked_sub(&mut x2_enc[i - 1], &mut y2_enc[j - 1]);
                sks.unchecked_scalar_add_assign(&mut eq2, 16);

                sks.unchecked_scalar_mul_assign(&mut eq2, 2);
                sks.unchecked_add_assign(&mut eq2, &eq1);

                sks.apply_lookup_table_assign(&mut eq2, &lut_eq);

                let vin = v_matrix[i][j - 1].clone();
                let hin = h_matrix[i - 1][j].clone();

                let v1 = sks.unchecked_scalar_add(&vin, 1);
                let h1 = sks.unchecked_scalar_add(&hin, 1);

                let key1 = sks.unchecked_scalar_mul(&h1, 3);
                let key12 = sks.unchecked_add(&key1, &eq2);

                let key = sks.unchecked_add(&key12, &v1);
                let lut = sks.generate_lookup_table(|x| lut_vec[x as usize]);

                let mut ct_res = sks.apply_lookup_table(&key, &lut);
                sks.unchecked_scalar_add_assign(&mut ct_res, 16);

                v_matrix[i][j] = sks.unchecked_sub(&ct_res, &hin);
                h_matrix[i][j] = sks.unchecked_sub(&ct_res, &vin);

            }
        }
        let elapsed = t.elapsed().as_secs_f64();
        println!("Round {i} \t Roundtime: {elapsed}");
    }

    let t_el = start.elapsed().as_secs_f32();


    let mut h_dec: Vec<Vec<u64>> = Vec::<Vec<u64>>::with_capacity(xlen);
    let mut v_dec: Vec<Vec<u64>> = Vec::<Vec<u64>>::with_capacity(xlen);

    for i in 0..xlen {
        let mut h_dec_row = Vec::<u64>::with_capacity(ylen);
        let mut v_dec_row = Vec::<u64>::with_capacity(ylen);

        for j in 0..ylen {
            let h_dec: u64 = cks.decrypt(&h_matrix[i][j]);
            h_dec_row.push(h_dec);
            let v_dec: u64 = cks.decrypt(&v_matrix[i][j]);
            v_dec_row.push(v_dec);
        }
        
        h_dec.push(h_dec_row);
        v_dec.push(v_dec_row);
    }

    // Non diagonal score
    // let mut score = 0;

    // for j in 0..ylen {
    //     if h_dec[xlen - 1][j] > 8 {
    //         score += &h_dec[xlen - 1][j] - 16;
    //     } else {
    //         score += &h_dec[xlen - 1][j];
    //     }
    // }

    // score += m as u64;


    // Diagonal score
    let mut diag_score = 0;

    for i in 1..m+1 {
        if h_dec[i][i] > 8 {
            diag_score += &h_dec[i][i] - 16;
        } else {
            diag_score += &h_dec[i][i];
        }
    }

    for i in 0..m {
        if v_dec[i+1][i] > 8 {
            diag_score += &v_dec[i+1][i] - 16;
        } else {
            diag_score += &v_dec[i+1][i];
        }
    }


    let prev_vec = levenshtein_plain(&x, &y);
    println!("Outcome of the plain Levenshtein distance: {}", prev_vec[m]);
    
    println!("Outcome of the Leuvenshtein distance: {}", diag_score);
    println!("Threshold: {th}");

    println!("Leuvenshtein Timing: {}", t_el);


}