//! Typst Evaluation

use comemo::Track;
use typst::World;
use typst::diag::SourceResult;
use typst::engine::{Engine, Route, Sink, Traced};
use typst::foundations::{Context, Func, Module, Value};
use typst::introspection::Introspector;
use typst::syntax::Source;
pub use typst_eval::*;

//ID10T start
use crate::utils::Protected;
//ID10T end

//ID10T start
use typst_library::introspection::EmptyIntrospector;
//ID10T end

/// Evaluates a source file and return the resulting module.
pub fn eval_compat(world: &dyn World, source: &Source) -> SourceResult<Module> {
    let route = Route::default();
    let traced = Traced::default();
    let mut sink = Sink::default();
    typst_eval::eval(
        &typst::ROUTINES,
        world.track(),
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
    )
}

/// The Typst Engine.
pub struct TypstEngine<'a> {
    /// The introspector to be queried for elements and their positions.
    //ID10T start
    pub introspector: Box<dyn Introspector>,
    //ID10T end
    /// May hold a span that is currently under inspection.
    pub traced: Traced,
    /// The route the engine took during compilation. This is used to detect
    /// cyclic imports and excessive nesting.
    pub route: Route<'static>,
    /// A push-only sink for delayed errors, warnings, and traced values.
    ///
    /// All tracked methods of this type are of the form `(&mut self, ..) ->
    /// ()`, so in principle they do not need validation (though that
    /// optimization is not yet implemented in comemo).
    pub sink: Sink,
    /// The environment in which typesetting occurs.
    pub world: &'a dyn World,
}

impl<'a> TypstEngine<'a> {
    /// Creates a new Typst Engine.
    pub fn new(world: &'a dyn World) -> Self {
        Self {
            //ID10T start
            introspector: Box::new(EmptyIntrospector {}),
            //ID10T end
            traced: Traced::default(),
            route: Route::default(),
            sink: Sink::default(),
            world,
        }
    }

    /// Creates the engine.
    pub fn as_engine(&'a mut self) -> Engine<'a> {
        Engine {
            routines: &typst::ROUTINES,
            world: self.world.track(),
            //ID10T start
            introspector: Protected::new(self.introspector.as_ref().track()),
            //ID10T end
            traced: self.traced.track(),
            sink: self.sink.track_mut(),
            route: self.route.clone(),
        }
    }

    /// Applies a function.
    pub fn apply(&'a mut self, func: &Func, ctx: Context, args: Vec<Value>) -> SourceResult<Value> {
        func.call(&mut self.as_engine(), ctx.track(), args)
    }

    /// Calls a function.
    pub fn call(&'a mut self, func: &Func, ctx: Context) -> SourceResult<Value> {
        self.apply(func, ctx, vec![])
    }
}
