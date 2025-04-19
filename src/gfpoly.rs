/* 
    brian fallaria
    Date: April 19 2025
*/
use std::fmt;

pub struct GFPoly {
    which_modulo: u16,
    coef_vals: Vec <u8>,
    alpha_pows: Vec <u8>,
}

impl GFPoly {
    pub fn with_coefs(coef_vals: Vec<u8>, which_modulo: u16) -> Self {
        let mut poly = GFPoly {
            which_modulo,
            coef_vals,
            alpha_pows: vec![0; 256],
        };
        
        poly.generate_pows();
        poly
    }

    pub fn generate_pows(&mut self) {
        // Initialize the power table
        self.alpha_pows[0] = 1; // α^0 = 1
        
        let mut current: u16 = 2; // α^1 = 2
        self.alpha_pows[1] = current as u8;
        
        // calculate α^i for i = 2 to 255 and store in the table
        for i in 2..256 {
            // multiply by α (2) in GF(2^8)
            current = current << 1;
            
            // if exceeds 255, reduce it modulo the polynomial
            if current > 255 {
                current ^= self.which_modulo;
            }
            
            self.alpha_pows[i] = current as u8;
        }
    }

    pub fn pow(&self, exp: u8) -> Option<u8> {
        Some(self.alpha_pows[exp as usize])
    }

    pub fn log(&self, val: u8) -> Option<u8> {
        if val == 0 {
            return None; // log₍α₎(0) is undefined
        }
        
        for i in 0..256 {
            if self.alpha_pows[i] == val {
                return Some(i as u8);
            }
        }
        
        Some(0 as u8)
    }
}

impl fmt::Display for GFPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for (i, &coef) in self.coef_vals.iter().enumerate() {
            // for all but the first term, print a plus sign
            if coef == 0 {
                continue;
            }
            if !first {
                write!(f, " + ")?;
            }
            first = false;
            
            // format the coefficient
            if coef == 1 {
                if i == 0 {
                    write!(f, "1")?;
                }
                // for x^1 or higher with coefficient 1, don't print the coefficient
            } else {
                let log_val = self.log(coef);
                
                if let Some(log_val) = log_val { 
                    if log_val == 0 {
                        if i == 0 {
                            write!(f, "1")?;
                        } else {
                            write!(f, "")?;
                        }
                    } else if log_val == 1 {
                        if i == 0 {
                            write!(f, "1")?;
                        } else {
                            write!(f, "a ")?;
                        }
                    } else {
                        if i == 0 {
                            write!(f, "a^{}", log_val)?;
                        } else {
                            write!(f, "a^{} ", log_val)?;
                        }
                    }
                } else {
                    // This shouldn't happen for valid input
                    write!(f, "?")?;
                }
            }
            
            // Format the x term
            if i > 0 {
                if i == 1 {
                    write!(f, "x")?;
                } else {
                    write!(f, "x^{}", i)?;
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod gfpoly {
        use super::*;

        #[test]
        fn test_init_sample() {
            let obj = GFPoly::with_coefs(vec![1], 285);

            assert_eq!(obj.coef_vals.len(), 1);
            assert_eq!(obj.alpha_pows.len(), 256);
            assert_eq!(obj.which_modulo, 285);
        }

        #[test]
        fn test_pow_sample() {
            let obj = GFPoly::with_coefs(vec![], 285);

            assert_eq!(obj.pow(0), Some(1));
            assert_eq!(obj.pow(1), Some(2));
            assert_eq!(obj.pow(72), Some(101));
        }

        #[test]
        fn test_log_sample() {
            let obj = GFPoly::with_coefs(vec![], 283);

            assert_eq!(obj.log(1), Some(0));
            assert_eq!(obj.log(2), Some(1));
            assert_eq!(obj.log(3), Some(0));
        }

        #[test]
        fn test_display_sample() {
            let obj = GFPoly::with_coefs(vec![0, 0, 12, 4, 0, 20], 285);

            assert_eq!(format!("{}", obj), "a^27 x^2 + a^2 x^3 + a^52 x^5");
        }
    }
}