// Ownership Basics
// Each value has a variable that's its owner
// A value can have only one owner at a time.
// If the owner goes out of scope, the value is cleaned up.
// this concept is similar to the java's JVM garbage collection
// which means once we create an instance and all of the reference to that instance is get out of context and scope
// that means this instance can be collected and released by the JVM's garbage.

// and from the lecturer it seems that Rust organize the reference and the isntance the same way which is
// similar to the JVM that holds the references(like C's pointer) to the memory's stack space
// and let heap hold the value or the instance memory
// when we say owner it means the references that store in the stack space, and the reference hold's the instance's metadata info

fn main() {
    let s1 = String::from("world");
    let s2 = s1;
    // here s1-> String_Heap("world") instance
    // s2 -> String_Heap("world")

    // here s1-> String_Heap("world") s2->String_Heap("world")
    // and s3-> String_Heap("world") but this String_Heap is a new allocated space different from the previous instance
    // let s3 = s1.clone_from(&s1);
}
