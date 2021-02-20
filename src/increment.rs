use std::vec::Vec;

static mut NUMBERS: Vec<i64> = Vec::new();
static mut SUM:i64 = 0;
static mut LENGTH:i64 = 0;

pub fn add_number(n:i64) -> i64{
    unsafe{
        NUMBERS.push(n);
        LENGTH += 1;
        SUM += n;
        SUM / LENGTH
    }
}

pub fn number_string() -> String{
    let mut out: String = "[ ".to_string();
    unsafe{
        for _n in NUMBERS.iter() {
            out += &_n.to_string();
            out += " ";
        }
    }
    out += "]";
    out
}