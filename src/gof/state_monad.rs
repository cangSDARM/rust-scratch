/// HKT: 将类型映射到类型
/// 如：我们可以把 Vec 视为"类型的函数"，给它输入一个类型，它会返回一个类型出来
pub trait HKT {
    type Higher<T>;
}

/// Monad: 实现了pure、flatMap的trait
pub trait Monad<'a, A>: HKT {
    /// pure要求返回一个包含参数类型内容的包裹
    fn pure(v: A) -> Self;
    /// flatMap要求把值经过f以后，其深度不增加. 如 flatMap([1, 2], |x| vec![x+1, x+2]) == [2, 3, 3, 4]
    fn flatMap<F: 'a, B>(self, f: F) -> Self::Higher<B> where F: Fn(A) -> Self::Higher<B>;
}

/// 包含 `run_state` 函数的 Monad。
/// 通过组合可以使变化的状态在逻辑间传递
pub struct State<'a, S, A> {
    /// `run_state` 将某个初态映射到(终值,末态)，即 `S -> (A, S)`
    pub run_state: Box<dyn 'a + Fn(S) -> (A, S)>,
}

impl<'a, S: 'a + Clone, A: 'a> State<'a, S, A> {
    /// run run_state
    pub fn run(self, state: S) -> (A, S) {
        (self.run_state)(state)
    }
    /// run run_state，并获取其value
    pub fn eval(self, state: S) -> A {
        (self.run_state)(state).0
        /// === self.run(state).0
    }
}

impl<'a, S, A> HKT for State<'a, S, A> {
    /// T -> ('a, S, T)
    type Higher<T> = State<'a, S, T>;
}

impl<'a, S: 'a, A: 'a + Clone> Monad<'a, A> for State<'a, S, A> {
    /// v -> (S -> (v, S))
    fn pure(v: A) -> Self {
        State {
            run_state: Box::new(move |state: S| (v.clone(), state)),
        }
    }

    /// f: (va) -> State<S, vb>
    /// run_state: (S) -> (V, S)
    fn flatMap<F: 'a, B>(self, f: F) -> Self::Higher<B> where F: Fn(A) -> Self::Higher<B> {
        State {
            run_state: Box::new(move |state: S| {
                /// get state1
                let (interm_value, interm_state) = (self.run_state)(state);
                /// input state1 to f, got state2. (why not return state2?, becuase it's lose the state1's state)
                /// then use state2's run_state to get final state
                (f(interm_value).run_state)(interm_state)
            }),
        }
    }
}

/// 丢弃Value，返回 (State, State)
pub fn get<'a, S: Clone>() -> State<'a, S, S> {
    State { run_state: Box::new(|state: S| (state.clone(), state)) }
}

/// 获取一个State
/// 清空Value, 只保留其State
pub fn put<'a, S: 'a + Clone>(state: S) -> State<'a, S, ()> {
    State { run_state: Box::new(move |_| ((), state.clone())) }
}

/// 类似于put, 允许动态的指定一个 State -> State 的函数，
/// 并返回清空Value后的State
pub fn modify<'a, S: 'a + Clone>(f: impl Fn(S) -> S + 'a) -> State<'a, S, ()> {
    State::flatMap(
        get(),
        move |x| put(f(x))
    )
}

#[cfg(test)]
mod test {
    /// StateMonad 优势：
    /// 仅使用了不可变对象
    /// 线性复杂度
    /// 可组合、可递归
    fn fib(n: u64) -> State<'static, (u64, u64), u64> {
        match n {
            0 => State::flatMap(
                get(),
                |x: (u64, u64)| { State::pure(x.0) }
            ), // -> State<(u64, u64), 0>
            _ => State::flatMap(
                modify(|x: (u64, u64)| { (x.1, x.0 + x.1) }),
                move |_| fib(n - 1)
            ) // -> State<fib(n-1), u64>
        }
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(7).eval((0, 1)), 13);
    }
}
