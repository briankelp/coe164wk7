mod gfpoly;

use std::io;
use crate::gfpoly::GFPoly;

fn process_io<R, W>(mut reader: R, mut writer: W) -> io::Result<()>
where
    R: io::BufRead,
    W: io::Write {

    let mut str_in = String::new();

    str_in.clear();
    reader.read_line(&mut str_in)
        .expect("Invalid input");
    let n_testcases: usize = str_in.trim().parse().expect("Invalid number");

    for t in 1..=n_testcases {
        str_in.clear();
        reader.read_line(&mut str_in)
            .expect("Invalid input");
        
        let values: Vec<u16> = str_in
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        
        let n = values[0] as usize;
        let modulo = values[1];
        
        let coefficients: Vec<u8> = values[2..(n+2)]
            .iter()
            .map(|&val| val as u8)
            .collect();
        
        let poly = GFPoly::with_coefs(coefficients, modulo);
        
        writeln!(writer, "Message #{}: {}", t, poly)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    process_io(io::stdin().lock(), io::stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sample() {
        let t_in = "\
        2\n\
        11 285 32 2 101 10 97 197 15 47 134 74 5\n\
        11 283 32 2 101 10 97 197 15 47 134 74 5\n\
        ";

        let t_out = "\
        Message #1: a^5 + a x + a^72 x^2 + a^51 x^3 + a^66 x^4 + a^123 x^5 + a^75 x^6 + a^69 x^7 + a^99 x^8 + a^37 x^9 + a^50 x^10\n\
        Message #2: a^5 + a x + x^2 + x^3 + a^35 x^4 + a^28 x^5 + x^6 + a^15 x^7 + x^8 + a^39 x^9 + x^10\n\
        ";

        let mut wt_in = io::Cursor::new(t_in.as_bytes());
        let mut wt_out: io::Cursor <Vec <u8>> = io::Cursor::new(vec![]);

        process_io(&mut wt_in, &mut wt_out);

        let mut begin_idx = 0usize;
        for (i, each_line) in t_out.split('\n').enumerate() {
            let end_idx = if begin_idx + each_line.len() > wt_out.get_ref().len() {
                wt_out.get_ref().len()
            }
            else {
                begin_idx + each_line.len()
            };

            println!("{} {} {}", begin_idx, each_line.len(), wt_out.get_ref().len());

            let ts = String::from_utf8(wt_out.get_ref()[begin_idx..end_idx].to_vec())
                .expect("Cannot convert to UTF-8");

            assert_eq!(ts, each_line, "at line {}", i);
            begin_idx += each_line.len() + 1;
        }
    }
}