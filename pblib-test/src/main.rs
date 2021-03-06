mod unsigned_integers;
mod signed_integers;
mod floating_points;
mod fixed_length_strings;
mod dynamic_strings;

fn main() {    

    println!("{:=^80}", " UNSIGNED INTEGERS ");
    
    println!("{:-^80}", " BYTE ");
    unsigned_integers::test_bytes();
    
    println!("{:-^80}", " WORD ");
    unsigned_integers::test_words();
    
    println!("{:-^80}", " DWORD ");
    unsigned_integers::test_dwords();

    println!("{:=^80}", " SIGNED INTEGERS ");
    
    println!("{:-^80}", " INTEGER ");
    signed_integers::test_integers();
    
    println!("{:-^80}", " LONG ");
    signed_integers::test_longs();
    
    println!("{:-^80}", " QUAD ");
    signed_integers::test_quads();

    println!("{:=^80}", " FLOATING POINTS ");
    
    println!("{:-^80}", " SINGLE ");
    floating_points::test_singles();
    
    println!("{:-^80}", " DOUBLE ");
    floating_points::test_doubles();

    println!("{:=^80}", " FIXED LENGTH STRINGS ");    

    println!("{:-^80}", " STRINGZ/ASCIIZ ");
    fixed_length_strings::test_stringzs();

    println!("{:=^80}", " DYNAMIC STRINGS ");    

    println!("{:-^80}", " STRING ");
    dynamic_strings::test_strings();    
}
