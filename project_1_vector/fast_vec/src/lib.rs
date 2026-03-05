use std::{fmt::{Display, Formatter}, ptr::{self, null_mut}};

use malloc::MALLOC;

pub struct FastVec<T> {
    ptr_to_data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> FastVec<T> {
    // Creating a new FastVec that is either empty or has capacity for some future elements.
    pub fn new() -> FastVec<T> {
        return FastVec::with_capacity(1);
    }
    pub fn with_capacity(capacity: usize) -> FastVec<T> {
        return FastVec {
            ptr_to_data: MALLOC.malloc(size_of::<T>() * capacity) as *mut T,
            len: 0,
            capacity: capacity,
        };
    }

    // Retrieve the FastVec's length and capacity
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let ptr = self.ptr_to_data.add(i);
                let element = ptr::read(ptr);
                v.push(element);
            }
        }
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
        return v;
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> FastVec<T> {
        let mut fast_vec: FastVec<T> = FastVec::with_capacity(vec.len());
        for element in vec {
            unsafe {
                let ptr = fast_vec.ptr_to_data.add(fast_vec.len);
                ptr::write(ptr, element);
            }
            fast_vec.len = fast_vec.len + 1;
        }
        return fast_vec;
    }

    // Student 1 and Student 2 should implement this together
    // Use the project handout as a guide for this part!
    pub fn get(&self, i: usize) -> &T {
        //todo!("implement get!");
        unsafe {
            if i >= self.len{
                panic!("FastVec: get out of bounds")
            } else {
                let ptr = self.ptr_to_data.add(i); 
            // the ptr stores the first address of the first element. 
            // if we want the i'th element, we perform pointer arithmetic (.add()) to move i*datasize addresses down.
            return &*ptr;
            }
        }
    }

    // Student 2 should implement this.
    pub fn push(&mut self, t: T) {
        if self.len == self.capacity {
            todo!("implement growing the vector by doubling the size!");
        } else {
            todo!("implement pushing t directly since the vector still has capacity!");
        }
    }

    // Student 1 should implement this.
    pub fn remove(&mut self, i: usize) {
        //todo!("implement remove");
        if i >= self.len{
            panic!("FastVec: remove out of bounds")
            //panic if the person is trying to reach out of range
        } else{
            unsafe {
                let ptr_remove = self.ptr_to_data.add(i);
                //I am getting the pointer of the i that I want to delete by adding i steps to the
                //starting pointer
                let old_variable = ptr::read(ptr_remove);
                // just removing the item in the index i
                for n in (i+1)..self.len{
                    // I am asking the loop to start from the index that is right after the one
                    // we deleted. It will safely go until the end of the vector because of len
                    let move_this = ptr::read(self.ptr_to_data.add(n));
                    // Deleting the item and moving it to variable
                    ptr::write(self.ptr_to_data.add(n-1),move_this);
                    // Now I am moving that variable to it's new position which is the one before
                }
                self.len = self.len - 1;
                // since we need to keep track of len I am just updating it.
            }
        }
    }

    // This appears correct but with further testing, you will notice it has a bug!
    // Student 1 and 2 should attempt to find and fix this bug.
    // Hint: check out case 2 in memory.rs, which you can run using
    //       cargo run --bin memory
    pub fn clear(&mut self) {
        MALLOC.free(self.ptr_to_data as *mut u8);
        self.ptr_to_data = null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}

// Destructor should clear the fast_vec to avoid leaking memory.
impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// This allows printing FastVecs with println!.
impl<T: Display> Display for FastVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FastVec[")?;
        if self.len > 0 {
            for i in 0..self.len()-1 {
                write!(f, "{}, ", self.get(i))?;
            }
            write!(f, "{}", self.get(self.len - 1))?;
        }
        return write!(f, "]");
    }
}