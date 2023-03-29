use core::marker::PhantomData;

struct Component<T>(PhantomData<T>);

struct Wrapper<T>(T);

impl<'a, Q: 'a> AsBorrow<'a> for Q {
    type Borrowed = QueryBorrow<'a, Q>;
}

pub struct QueryData<'a, Q> {
    marker: PhantomData<&'a Q>,
}

struct QueryBorrow<'w, Q> {
    marker: PhantomData<&'w Q>,
}

pub trait AsBorrow<'a> {
    /// The dereference target
    type Borrowed: 'a;
}

/// A callable function with 'self lifetime
/// Is not reproducible with Fn trait
trait SystemFn<'this, Args> {
    // fn execute(&'this mut self, args: Args) -> Ret;
}

pub fn build<Args, Func>(data: Args, func: Func)
where
    Func: for<'this, 'a> SystemFn<'this, Args>,
{
}

impl<'this, Func, A> SystemFn<'this, A> for Func
where
    for<'x> A: AsBorrow<'x>,
    for<'x> Func: Fn(<A as AsBorrow<'x>>::Borrowed),
{
}

fn main() {
    let component: Component<i32> = Component(PhantomData);

    let query = Wrapper(component);

    // Component instead of the actual Wrapper<Component<i32>, i32>>
    let system = build(query, |query: QueryBorrow<Component<i32>>| {});
}
