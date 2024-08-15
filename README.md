# Rust-specialization-attempts
I may have gotten Rust to more-or-less agree to let me have multiple different default implementations for trait functions, depending on which other traits are implemented. There are some quirks, but they're not *that* bad. 

The general structure follows this format (see example files for more specifics):
```Rust
trait GenericBaseline {}

trait AdmitsOptimizationOne: GenericBaseline {
    fn get_value() -> i8;
}

trait ParameterizedOptimizations<OptimizationDetails>: GenericBaseline {
    fn get_value() -> i8;
}

impl<T: GenericBaseline> ParameterizedOptimizations<()> for T {
    fn get_value() -> i8 {
        return 0;
    }
}

struct OptimizationOneTag {}

impl<T: AdmitsOptimizationOne> ParameterizedOptimizations<OptimizationOneTag> for T {
    fn get_value() -> i8 {
        return <T as AdmitsOptimizationOne>::get_value();
    }
}

trait UnifyingCase: GenericBaseline {
    type OptimizationDetails;
    fn get_value() -> i8 where Self: ParameterizedOptimizations<Self::OptimizationDetails> {
        return <Self as ParameterizedOptimizations<Self::OptimizationDetails>>::get_value()
    }
}
```

The user of the traits will implement `GenericBaseline` functionality manually. They will implement `AdmitsOptimizationOne` if applicable. Then they will implement `UnifyingCase`, the only requirement for which implementation is to specify which default case the program is to use for the type in question. The user will never need to directly interact with `ParameterizedOptimizations`, which only serves as an intermediate to connect the user's input traits (here is the basic functionality I can provide) with our output trait (`UnifyingCase`, which has the fully fleshed-out trait methods). Again, see the example file for a look at how this implementation might work in practice. 
