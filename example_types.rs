use super::example_traits::{BasicHasTwoValues, ImprovedHtv, ManuallyImplementedFlag, StoresValueDifference, StoresValueDifferenceTag};


// Some structs that hold two values. 
struct TwoValueHolder {
    value_one: i8,
    value_two: i8,
}

struct TwoValueAndDifferenceHolder {
    value_one: i8,
    value_two: i8,
    difference: i8,
}

struct ValueAndOffsetHolder {
    base_value: i8,
    offset: i8,
}

// We need to give as input our implementations of the basic trait methods, plus any optimizations. 
// Later we will get (as output from the traits file) access to a different trait that unifies all 
// our cases and has fully fleshed-out, specialized methods. We can choose which provided base case 
// we want to use for the methods of a given type, or we can choose to implement our own methods. 

// The first struct only has basic functionality. 
impl BasicHasTwoValues for TwoValueHolder {
    fn get_value_one(&self) -> i8 {
        return self.value_one;
    }

    fn get_value_two(&self) -> i8 {
        return  self.value_two;
    }
}

// The second struct admits an optimization . 
impl BasicHasTwoValues for TwoValueAndDifferenceHolder {
    fn get_value_one(&self) -> i8 {
        return self.value_one;
    }

    fn get_value_two(&self) -> i8 {
        return self.value_two;
    }
}

impl StoresValueDifference for TwoValueAndDifferenceHolder {
    fn get_value_difference(&self) -> i8 {
        return self.difference;
    }
}

// The third struct works the same way as the second one for now. 
impl BasicHasTwoValues for ValueAndOffsetHolder {
    fn get_value_one(&self) -> i8 {
        return self.base_value;
    }

    fn get_value_two(&self) -> i8 {
        return self.base_value + self.offset;
    }
} 

impl StoresValueDifference for ValueAndOffsetHolder {
    fn get_value_difference(&self) -> i8 {
        return self.offset;
    }
}

// All three types perform all the functionality we wanted from HasTwoValues, 
// and can either use routines taken from the multiple base cases we wrote in 
// the other file or their own manually implemented routines. 
impl ImprovedHtv for TwoValueHolder {
    type OptimizationDetails = ();
}

impl ImprovedHtv for TwoValueAndDifferenceHolder {
    type OptimizationDetails = StoresValueDifferenceTag;
}

impl ImprovedHtv for ValueAndOffsetHolder {
    type OptimizationDetails = ManuallyImplementedFlag;
    fn first_value_smaller_q(&self) -> bool where Self: ParametrizedHtv<Self::OptimizationDetails> {
        return self.offset < 0;
    }
}
