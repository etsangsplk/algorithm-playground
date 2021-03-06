// Given a vec of u8 return the corresponding base64 string
// Requires more testing
const ALL64: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];


fn push_four(s: &mut String, r: usize){
    let mask:usize = 63;
    s.push(ALL64[r >> 18 & mask].clone());
    s.push(ALL64[r >> 12 & mask].clone());
    s.push(ALL64[r >> 6 & mask].clone());
}


fn to_base64(v: &[u8]) -> String {
    let l = v.len() / 3;
    let m = v.len() % 3;
    let mut s = String::new();

    for i in 0..l {
        let x = 3*i;
        let (a, b, c) = (v[x] as usize, v[x+1] as usize, v[x+2] as usize);
        let r:usize = (a << 16) + (b << 8) + c;
        push_four(&mut s, r);
    }

    if m == 0 { return s; }
    
    let mask = 63;
    let r;
    
    if m == 2 {
        let (a, b) = (v[v.len()-2] as usize, v[v.len()-1] as usize);
        r = (a << 16) + (b << 8);
        s.push(ALL64[r >> 18 & mask].clone());
        s.push(ALL64[r >> 12 & mask].clone());
        s.push(ALL64[r >> 6 & mask].clone());
        
    } else {
        r = (v[v.len()-1] as usize) << 16;
        s.push(ALL64[r >> 18 & mask].clone());
        s.push(ALL64[r >> 12 & mask].clone());
        s.push('=');
    }
    
    s.push('=');
    s
}


fn main(){

    let a: Vec<u8> = vec![84, 101, 115, 116, 10]; 
    let b: Vec<u8> = vec![97, 110, 121, 32, 99, 97, 114, 110, 97, 108, 32, 112, 108, 101, 97, 115, 117, 114, 101];

    let v: Vec<u8> = vec![
        73, 39, 109, 32, 107, 105, 108, 108, 105, 110, 103, 32, 121, 111, 117, 114,
        32, 98, 114, 97, 105, 110, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 105,
        115, 111, 110, 111, 117, 115, 32, 109, 117, 115, 104, 114, 111, 111, 109
        ];

    let w: Vec<u8> = vec![77, 97]; 
    let x: Vec<u8> = vec![77]; 

    println!("{}", to_base64(&a)); // VGVzdAo=
    println!("{}", to_base64(&b)); //YW55IGNhcm5hbCBwbGVhc3VyZQ==

    println!("{}", to_base64(&v)); // "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    println!("{}", to_base64(&w)); // TWE=
    println!("{}", to_base64(&x)); // TQ==
}
