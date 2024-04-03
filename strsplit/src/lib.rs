
// #![warn(missing_debug_implementations,rust_2018_idioms,missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack,D> {//multiple lifetimes are only used when u have to store multiple references n it shouldnt be same
    remainder:Option<&'haystack str>,
    delimiter:D,
}
pub trait Delimiter {
    fn find_next (&self,s:&str) -> Option<(usize,usize)>;
}

impl <'haystack,D> StrSplit <'haystack,D> 
 {   //strsplit lives upto the lifetime of values in new fn
    pub fn new(haystack:&'haystack str,delimiter:D) -> Self { //This means that the strings these references point to 
                                                            //must live at least as long as the StrSplit struct.
        Self{
            remainder:Some(haystack),
            delimiter,
        }

    }
}
//iterator trait
//let x:StrSplit 
//for part in x
impl <'haystack,D> Iterator for StrSplit<'haystack,D>
where
D:Delimiter,
{ //lifetime 'a if even you drop the value u can use it bcoz of same lifetime
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {  //it will keep calling next for the time it is returning some 
     //  if let Some(ref mut remainder)= self.remainder
       let remainder = self.remainder.as_mut()?;
     //as_mut:takes impl<T> Option<T>{fn as_mut(&mut self)->Option<&mut T>}
       
        if let Some((delim_start,delim_end)) = self.delimiter.find_next(remainder){
        
          let until_delimiter = &remainder[..delim_start];
         //*  */remainder on left is &mut &'a str n on right &'a str as theese are not of same type so * is used
          *remainder = &*remainder[delim_end..]; // '_ anonymous lifetimes is when u tell the compiler ot guess the lifetime 
                                                                                 // n it only works when there is only 1 guess
         
          Some(until_delimiter)
        }else {
            self.remainder.take()
        }
        
        } 
    }


impl Delimiter for &str {
    fn find_next (&self,s:&str) -> Option<(usize,usize)> {
        s.find(self).map(|start|
            (start,start + self.len())
        )
    }
}
    

        
    


// lifetime of a value is untill that value is moved if it isnt moved the lifetime is static


fn until_char(s:&str,c:char)->&'_ str{
    let delim = format!("{}",c);
 StrSplit::new(s,&*delim)
 .next()
 .expect("strsplit always gives atleast one result")

}

#[test]
fn until_char_test(){
    assert_eq!(until_char("hello world",'o'),"hell");
}




// str -> [char] doesnt have a size,can point ot anything stack,heap,static memomy 
//&str-> &[char] it remembers where the string starts and how long it is
//string = vec<char>,heap allocated and dynamically expandable 
//string -> &str cheap n uses asRef  but &str -> String -- expensive and uses somewhat of a clone(memcopy)is harder only way u can do it is doing heap allocation and copying all the chars




#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}