use core::marker::PhantomData;

struct Component<T> {
    marker: PhantomData<T>,
}

struct Wrapper<T, V> {
    component: T,
    value: V,
}

impl<'a, 'w, Q> AsBorrow<'a> for QueryData<'w, Q>
where
    Q: 'static,
    QueryBorrow<'w, Q>: 'a,
{
    type Borrowed = QueryBorrow<'w, Q>;
}

pub trait SystemData<'a> {
    type Value;
}

pub struct Query<Q> {
    fetch: Q,
}

impl<Q> Query<Q> {
    pub fn new(fetch: Q) -> Self {
        Self { fetch }
    }
}

impl<'a, Q> SystemData<'a> for Query<Q>
where
    Q: 'static,
{
    type Value = QueryData<'a, Q>;
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
trait SystemFn<'this, Args, Ret> {
    // fn execute(&'this mut self, args: Args) -> Ret;
}

pub fn build<Args, Func>(data: Args, func: Func)
where
    Args: for<'a> SystemData<'a> + 'static,
    Func: for<'this, 'a> SystemFn<'this, <Args as SystemData<'a>>::Value, ()>,
{
}

impl<'this, Func, Ret, A> SystemFn<'this, A, Ret> for Func
where
    for<'x> A: AsBorrow<'x>,
    for<'x> Func: FnMut(<A as AsBorrow<'x>>::Borrowed) -> Ret,
{
}

fn main() {
    let component: Component<f32> = Component {
        marker: PhantomData,
    };

    let query = Query::new(Wrapper {
        component,
        value: 0.0,
    });

    // Component instead of the actual Wrapper<Component<f32>, f32>>
    let system = build(query, |query: QueryBorrow<Component<f32>>| {});
}
