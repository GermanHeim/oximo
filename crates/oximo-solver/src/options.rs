use std::time::Duration;

/// Options valid for every solver, including NLP backends.
/// Each backend's options struct embeds this via [`HasUniversal`]; the
/// [`UniversalOptionsExt`] blanket impl then provides typed builder setters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UniversalOptions {
    pub time_limit: Option<Duration>,
    pub threads: Option<u32>,
    pub verbose: Option<bool>,
}

/// Options specific to LP / MILP solvers. NLP backends do not embed this.
/// Each LP/MILP options struct embeds this via [`HasMip`]; the
/// [`MipOptionsExt`] blanket impl provides `mip_gap` and `presolve` setters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MipOptions {
    pub mip_gap: Option<f64>,
    pub presolve: Option<Presolve>,
}

/// Presolve strategy for LP / MILP backends.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Presolve {
    Off,
    On,
    Auto,
}

/// Implemented by every backend-specific options struct.
/// Gives the [`UniversalOptionsExt`] blanket impl access to the embedded
/// [`UniversalOptions`].
pub trait HasUniversal {
    fn universal(&self) -> &UniversalOptions;
    fn universal_mut(&mut self) -> &mut UniversalOptions;
}

/// Implemented by LP / MILP backend options structs.
/// Gives the [`MipOptionsExt`] blanket impl access to the embedded
/// [`MipOptions`].
pub trait HasMip {
    fn mip(&self) -> &MipOptions;
    fn mip_mut(&mut self) -> &mut MipOptions;
}

/// Builder setters available on every backend options struct.
pub trait UniversalOptionsExt: HasUniversal + Sized {
    #[must_use]
    fn time_limit(mut self, d: Duration) -> Self {
        self.universal_mut().time_limit = Some(d);
        self
    }

    #[must_use]
    fn threads(mut self, n: u32) -> Self {
        self.universal_mut().threads = Some(n);
        self
    }

    #[must_use]
    fn verbose(mut self, on: bool) -> Self {
        self.universal_mut().verbose = Some(on);
        self
    }
}

impl<T: HasUniversal> UniversalOptionsExt for T {}

/// Builder setters available only on LP / MILP backend options structs.
pub trait MipOptionsExt: HasMip + Sized {
    #[must_use]
    fn mip_gap(mut self, gap: f64) -> Self {
        self.mip_mut().mip_gap = Some(gap);
        self
    }

    #[must_use]
    fn presolve(mut self, p: Presolve) -> Self {
        self.mip_mut().presolve = Some(p);
        self
    }
}

impl<T: HasMip> MipOptionsExt for T {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    // A minimal stand-in that implements both traits so we can test the
    // blanket ext impls without pulling in any backend crate.
    #[derive(Default)]
    struct TestOpts {
        pub universal: UniversalOptions,
        pub mip: MipOptions,
    }
    impl HasUniversal for TestOpts {
        fn universal(&self) -> &UniversalOptions {
            &self.universal
        }
        fn universal_mut(&mut self) -> &mut UniversalOptions {
            &mut self.universal
        }
    }
    impl HasMip for TestOpts {
        fn mip(&self) -> &MipOptions {
            &self.mip
        }
        fn mip_mut(&mut self) -> &mut MipOptions {
            &mut self.mip
        }
    }

    #[test]
    fn universal_default_is_all_none() {
        let u = UniversalOptions::default();
        assert!(u.time_limit.is_none());
        assert!(u.threads.is_none());
        assert!(u.verbose.is_none());
    }

    #[test]
    fn mip_default_is_all_none() {
        let m = MipOptions::default();
        assert!(m.mip_gap.is_none());
        assert!(m.presolve.is_none());
    }

    #[test]
    fn universal_builder_chain() {
        let o = TestOpts::default().time_limit(Duration::from_secs(60)).threads(4).verbose(false);
        assert_eq!(o.universal.time_limit, Some(Duration::from_secs(60)));
        assert_eq!(o.universal.threads, Some(4));
        assert_eq!(o.universal.verbose, Some(false));
    }

    #[test]
    fn mip_builder_chain() {
        let o = TestOpts::default().mip_gap(0.01).presolve(Presolve::Off);
        assert_eq!(o.mip.mip_gap, Some(0.01));
        assert_eq!(o.mip.presolve, Some(Presolve::Off));
    }

    #[test]
    fn presolve_variants_are_distinct() {
        assert_ne!(Presolve::Off, Presolve::On);
        assert_ne!(Presolve::On, Presolve::Auto);
        assert_ne!(Presolve::Auto, Presolve::Off);
    }

    #[test]
    fn universal_options_clone_eq() {
        let a = TestOpts::default().threads(2).verbose(true);
        let b = UniversalOptions { threads: Some(2), verbose: Some(true), ..Default::default() };
        assert_eq!(a.universal, b);
        let c = a.universal.clone();
        assert_eq!(c, b);
    }

    #[test]
    fn last_set_wins_for_same_field() {
        // Builder overwrites the same field, last call wins.
        let o = TestOpts::default().threads(1).threads(8);
        assert_eq!(o.universal.threads, Some(8));
    }
}
