use std::fmt;

struct Sstring {
    data : Box<[char]>,
}

fn main() {
    let my_string = Sstring{data : Box::new(['h', 'e', 'l','l','o'] )};
    println!("{:?}",my_string );
}

impl fmt::Debug for Sstring {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result{
        let mut err_happened : bool = false;
        for i in self.data.iter(){
            match write!(f, "{}", i){
                Ok(_) => continue,
                Err(_) => {
                    err_happened = true;
                    break;}
            };
        }
        if err_happened {
            std::result::Result::Err(std::fmt::Error)
        }else{
        std::result::Result::Ok(())
    }
    }
}
