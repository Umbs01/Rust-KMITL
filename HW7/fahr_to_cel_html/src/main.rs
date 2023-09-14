
fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("<style>\ntable, td {{ border: 1px solid #000000; border-collapse: collapse; }} \n</style>");
    println!("");
    println!("<table>");
    println!("\t<tr>\n\t\t<th>Fahrenheit</th>\n\t\t<th>Celcius</th>\n\t</tr>");

    match args.len() {
        4 => {
            let mut start :i32 = args[1].parse().unwrap();
            let stop :i32 = args[2].parse().unwrap();
            let step :i32 = args[3].parse().unwrap(); 
            let n :i32 = (start + stop)/step;
            let d :i32 = (stop - start)/n;

            for _i in 0..n+1 {
                let c:f32 = fahr_to_cel_html::conv(start);
                println!("\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{:.2}</td>\n\t</tr>",start as f32, c);
                start += d;
            }
        }
        _ => {
            err()
        }
    }
    println!("</table>");
}

fn err() {
    println!("Please input sufficient data")
}