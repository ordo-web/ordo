
pub trait Store {
    fn print(&self);
}


struct SingleStore<'a, State, Param> {
    state: State,
    actions: Vec<String>,
    reducer: fn(&'a State, String, Option<&'a Param>) -> State,
    param: Option<&'a Param>,
}



struct CombinedStore<'a, State, Param>  {
    stores: Vec<(String, SingleStore<'a, State, Param>)>,
}









