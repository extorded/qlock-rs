fn main() {
    let src = "fn main()\n{\n    let src = \"?\";\n    let src_n = src.len();\n    for i in 0..src_n {\n        if src.as_bytes()[i] == 63 {\n            for j in 0..src_n {\n                match src.as_bytes()[j] {\n                    b'\\n' => print!(\"\\\\n\"),\n                    b'\"' => print!(\"\\\"\"),\n                    b'\\' => print!(\"\\\\\\\\\"),\n                    _ => print!(\"{}\", src.as_bytes()[j] as char),\n                }\n            }\n        } else {\n            print!(\"{}\", src.as_bytes()[i] as char);\n        }\n    }\n}\n";
    let src_n = src.len();
    for i in 0..src_n {
        if src.as_bytes()[i] == 63 {
            for j in 0..src_n {
                match src.as_bytes()[j] {
                    b'\n' => print!("\\n"),
                    b'"' => print!("\\\""),
                    b'\\' => print!("\\\\"),
                    _ => print!("{}", src.as_bytes()[j] as char),
                }
            }
        } else {
            print!("{}", src.as_bytes()[i] as char);
        }
    }
}
