/*  

Rule #1 
Each input parameter gets its own lifetime. 
If the lifetime is not specified, then the lifetime of each parameter is different.

Rule #2
If there is only one input parameter, its lifetime is assigned to all the elided output lifetimes. 

Rule #3
If there are multiple input lifetimes, one of them is &self or &mut self, 
the lifetime of self is assigned to all elided output lifetimes.*/