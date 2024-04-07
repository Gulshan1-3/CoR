fn main() {
    let mut iter = vec!["a","b","c"].into_iter(); //Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
    while let Some(e) = iter.next(){


    }
}
