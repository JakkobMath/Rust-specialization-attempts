// Contrived example. 

// We want more functionality out of this trait. In particular, we want to know whether 
// value_1 < value_2 as quickly as possible, for some reason, and this wouldn't require evaluating 
// both quantities for all types. We could do this in the naive way and implement a more particular 
// version for every specific type for which the optimization exists, but this way will mimic 
// specialization more. Keep this trait skeletal for now. 
pub(crate) trait BasicHasTwoValues {
    fn get_value_one(&self) -> i8;
    fn get_value_two(&self) -> i8;
}

// One special case we might see: properly-formed values of these types store the difference 
// between their values already, and we can just check that stored quantity. 
pub(crate) trait StoresValueDifference: BasicHasTwoValues {
    // Should be in `value_two - value_one` format. 
    fn get_value_difference(&self) -> i8;
}

pub(crate) struct StoresValueDifferenceTag {}

// We now have some different varieties of value-storing traits. Let's temporarily combine them 
// loosely, like this. This is where we'll actually implement the additional functionality, since 
// the implementation we'll use depends on which case *of this trait* we fall under. 
trait ParametrizedHtv<T>: BasicHasTwoValues {
    fn first_value_smaller_q(&self) -> bool;
}

// All of our `BasicHasTwoValues` types fall into at least one case, 
// but there are more specialized cases as well. 

impl<T: BasicHasTwoValues> ParametrizedHtv<()> for T {
    fn first_value_smaller_q(&self) -> bool {
        // This is a valid default implementation, but requires calling the two methods above. 
        // A more efficient implementation exists for other types. 
        return match self.get_value_one().cmp(&self.get_value_two()) {
            std::cmp::Ordering::Less => true,
            _ => false,
        }
    }
}

impl<T: StoresValueDifference> ParametrizedHtv<StoresValueDifferenceTag> for T {
    fn first_value_smaller_q(&self) -> bool {
        return self.get_value_difference() < 0;
    }
}

// This tag tells us the user would like to implement some
// functionality for themselves rather than using our base cases. 
pub(crate) struct ManuallyImplementedFlag {}

impl<T: BasicHasTwoValues> ParametrizedHtv<ManuallyImplementedFlag> for T {
    // Will panic if the user tries to get away with not implementing the function. 
    fn first_value_smaller_q(&self) -> bool {
        todo!()
    }
}

// The first two public traits are the ones the user of this crate would actually implement. This 
// more-or-less auomatically implemented (per the above impls) trait is the one they will call methods from. 
pub(crate) trait ImprovedHtv: BasicHasTwoValues {

    type OptimizationDetails;
    
    // I don't know what I'm doing, but I assume this limits the user to working with this trait 
    // through the automatic impls provided above. They will implement BasicHasTwoValues and then 
    // possibly a method specific to getting their difference without calculating either, write 
    // an impl block for this trait, provide the type of optimization to use (here the unit type 
    // for no optimization), and then be able to call this optimized method that uses one of the 
    // base cases depending on which optimization they chose. They can also still override the 
    // method. We also provide in this file another "I'll optimize it myself" flag and similar 
    // impl blocks to the first two to support overriding more particularly, where the default 
    // implementation of first_value_smaller_q() is to immediately panic. 
    #[allow(private_bounds)] 
    fn first_value_smaller_q(&self) -> bool where Self: ParametrizedHtv<Self::OptimizationDetails> {
        return <Self as ParametrizedHtv<Self::OptimizationDetails>>::first_value_smaller_q(&self);
    }
}

