//#[derive(Clone,Copy)]
struct Bloop<T> { // <T> specifies that we have a generic parameter
    a: usize, 
    b: u64, 
    c: T, // the generic
}

// learn about enums
// https://github.com/spacejam/sled/blob/master/crates/sled/src/frag.rs
//#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//pub(crate) enum Frag {
    //Set(IVec, IVec), // these a called variants 
    //Del(IVec),
    //Merge(IVec, IVec),
    //Base(Node),
    //ParentMergeIntention(PageId),
    //ParentMergeConfirm,
    //ChildMergeCap,
//}


impl<T> Bloop<T> {
    fn gloop(&mut self) {
        self.a += 100;
        println!("Gloopin!");
    }
}

//impl Drop for Bloop {
    //fn drop(&mut self) {
        //println!("Dropping!");
    //}
//}

//fn f1(_bloop: &Bloop<u64>) { // can do this
fn f1<T>(_bloop: &Bloop<T>) {
    println!("hey I'm blimoasdap");
}

fn main() {
    println!("Hello, world!");
    //let bloop = Bloop{ a:5, b:55 }; // creates an immutable version of bloop
    let mut bloop = Bloop{ a:5, b:55, c:"gello" }; // this is actually a string-slice (not a String)
    //let bloop = bloop; // this "turns off" the mutability 
    bloop.gloop();
    f1(&bloop); // by passing in "by reference" aka using `&` this is a read only version
    bloop.gloop();
    println!("Hello, world!2");
    f1(&bloop);
    bloop.gloop();

    let boxed_bloop = Box::new(Bloop{ a:6, b:66, c:666 }); // exists on the heap  - kinda like a pointer but treat like a regular Bloop

    let mut v = vec![1, 2, 3]; 
    //println!("popping item from v: {:?}", v.pop());
    //println!("popping item from v: {:?}", v.pop());
    //println!("popping item from v: {:?}", v.pop());
    //println!("popping item from v: {:?}", v.pop());
    
    //for item_of_v in v { // this is actually consuming v 
    for item_of_v in &v { // doesn't consume v 
      dbg!(item_of_v);
    }

    match v.pop() {
        Some(what_got_popped_on_v) => println!("poppsed out this {:?} still has this {:?}",what_got_popped_on_v, v),
        None => println!("we got nuttin"),
    }

    // similar to a match
    if let Some(what_got_popped_on_v) = v.pop() { // ignores the None case 

    } else { // this would handle the None case

    }
}
